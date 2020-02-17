use log::*;

const LOGO: &str = r#"               __
              /'_ `\
  ___   _ __ /\ \L\ \  _ __
 /'___\/\`'__\/_> _ <_/\`'__\   {{ app }}
/\ \__/\ \ \/  /\ \L\ \ \ \/    v. {{ version }}
\ \____\\ \_\  \ \____/\ \_\    c. {{ commit }}
 \/____/ \/_/   \/___/  \/_/

"#;

pub struct Logo {
    pub app: &'static str,
    pub version: &'static str,
    pub commit: &'static str,
}

impl Logo {
    pub fn log(&self) {
        for line in self.lines() {
            info!("{}", line);
        }
    }

    pub fn lines(&self) -> Vec<String> {
        self.string()
            .lines()
            .map(ToOwned::to_owned)
            .collect()
    }

    pub fn string(&self) -> String {
        LOGO.replace("{{ app }}", self.app)
            .replace("{{ version }}", self.version)
            .replace("{{ commit }}", self.commit)
    }
}