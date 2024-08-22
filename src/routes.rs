use warp::Filter;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::models::{Portfolio, Project, SocialLinks};

#[derive(Deserialize, Serialize)]
pub struct FormInput {
    name: String,
    bio: String,
    contact: String,
    email: String,
    skills: String,
    education: String,
    experience: String,
    services: String,
    blog: String,
    project_name: String,
    project_desc: String,
    project_tech: String,
    github_link: Option<String>,
    live_demo: Option<String>,
    profile_photo: Option<String>, 
    linkedin_link: Option<String>, 
    twitter_link: Option<String>,  
    facebook_link: Option<String>, 
}

pub fn index() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path::end().map(|| {
        let html = include_str!("../templates/form.html");
        warp::reply::html(html)
    })
}

pub fn submit(handlebars: Arc<Handlebars<'static>>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::post()
        .and(warp::path("submit"))
        .and(warp::body::form())
        .map(move |form_input: FormInput| {
            // Convert comma-separated strings into vectors
            let skills: Vec<String> = form_input.skills.split(',')
                .map(|s| s.trim().to_string())
                .collect();

            let project_tech: Vec<String> = form_input.project_tech.split(',')
                .map(|s| s.trim().to_string())
                .collect();
            
            // Construct the Portfolio object with form data
            let github_link_clone = form_input.github_link.clone();

            let user = Portfolio {
                name: form_input.name,
                bio: form_input.bio,
                contact: form_input.contact,
                email: form_input.email,
                skills,
                education: form_input.education,
                experience: form_input.experience,
                services: form_input.services,
                blog: form_input.blog,
                projects: vec![Project {
                    name: form_input.project_name,
                    description: form_input.project_desc,
                    technologies: project_tech,
                    github_link: form_input.github_link,
                    live_demo: form_input.live_demo,
                }],
                social_links: SocialLinks {
                    github: github_link_clone,
                    facebook: form_input.facebook_link,
                    linkedin: form_input.linkedin_link,
                    twitter: form_input.twitter_link,
                },
                profile_photo: form_input.profile_photo,
            };            

            // Render the confirmation template with the user data
            let template = include_str!("../templates/confirmation.html");
            let html = handlebars.render_template(template, &user).unwrap();
            warp::reply::html(html)
        })
}


