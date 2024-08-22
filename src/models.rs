use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Portfolio {
    pub name: String,
    pub bio: String,
    pub contact: String,
    pub email: String,
    pub skills: Vec<String>,
    pub education: String,
    pub experience: String,
    pub services: String,
    pub blog: String,
    pub projects: Vec<Project>,
    pub social_links: SocialLinks,
    pub profile_photo: Option<String>, 
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub github_link: Option<String>,
    pub live_demo: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SocialLinks {
    pub github: Option<String>,
    pub linkedin: Option<String>,
    pub twitter: Option<String>,
    pub facebook: Option<String>,
}
