use std::str::SplitWhitespace;

#[derive(Debug, Copy, Clone)]
pub enum TestRequirement {
    TypecheckerPass,
    TypecheckerFail
}

impl TestRequirement {
    pub fn parse(script: String) -> Vec<TestRequirement> {
        let mut requirements = vec![];

        for line in script.lines() {
            if !line.starts_with("##") {
                continue;
            }

            let line = line.trim_start_matches("##").trim();
            let mut parts = line.split_whitespace();

            let keyword = match parts.next() {
                Some(keyword) => keyword,
                None => break,
            };

            let requirement = match keyword {
                "Typechecker:" => Self::parse_typechecker_status(parts),
                _ => {
                    eprintln!("Unknown requirement `{}`", keyword);
                    continue;
                },
            };

            if let Some(requirement) = requirement {
                requirements.push(requirement);
            }
        }

        requirements
    }

    fn parse_typechecker_status(mut parts: SplitWhitespace) -> Option<TestRequirement> {
        let status = match parts.next(){
            Some(next) => next,
            None => return None,
        };

        match status {
            "Pass" => Some(TestRequirement::TypecheckerPass),
            "Fail" => Some(TestRequirement::TypecheckerFail),

            _ => {
                eprintln!("Unknown typechecker status `{}`", status);
                None
            },
        }
    }
}