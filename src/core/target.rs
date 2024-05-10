use crate::daemon::service::OpenService;

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
pub enum PreTarget {
    Boot,
    Post,
    #[default]
    None,
}

#[derive(Default, Eq, PartialEq, Copy, Clone, Debug)]
pub enum PostTarget {
    Setup,
    Loop,
    Shell,
    #[default]
    None,
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Target {
    pub pre: PreTarget,
    pub post: PostTarget
}

impl Target {
    pub fn new(service: &OpenService) -> Self {
        let target: Vec<&str> = service.service.target.split(':').collect();
        
        let (pre, post) = (target[0], target[1]);
        
        let pre_target = match pre.to_lowercase().as_str() {
            "boot" => PreTarget::Boot,
            "post" => PreTarget::Post,
            _ => PreTarget::None,
        };
        
        let post_target = match post.to_lowercase().as_str() {
            "setup" => PostTarget::Setup,
            "loop" => PostTarget::Loop,
            "shell" => PostTarget::Shell,
            _ => PostTarget::None,
        };
        
        Self {
            pre: pre_target,
            post: post_target
        }
    }
}
