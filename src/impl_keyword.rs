struct User {
    name: String,
    email: String,
    is_active: bool,
}

impl User {
    fn get_name(&self) {
        println!("Nome: {}", self.name);
    }

    fn get_email(&self) {
        println!("Email: {}", self.email);
    }

    fn get_active_status(&self) {
        if self.is_active {
            println!("Usuário está ativo");
        } else {
            println!("Usuário não está ativo");
        }
    }

    fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
}

pub(crate) fn user_data(name: String) {
    let mut user = User {
        email: "test".to_string(),
        name,
        is_active: false,
    };

    user.get_name();

    user.set_name("Novo nome de teste".to_string());

    user.get_name();
    user.get_active_status();
    user.get_email();
}
