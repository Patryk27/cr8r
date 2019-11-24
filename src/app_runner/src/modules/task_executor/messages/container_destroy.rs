/**
debug!("Destroying container.");

        Command::new("/snap/bin/lxc")
            .args(&["stop", &self.container_name()])
            .output()
            .unwrap();

        debug!("Container `{}` destroyed.", self.container_name());
*/