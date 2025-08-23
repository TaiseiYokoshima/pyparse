use std::marker::PhantomData;

pub struct Person {
    name: String,
    email: String,
    phone: String,
}

struct Present;
struct Missing;

pub struct PersonBuilder<Name, Email, Phone> {
    name: Option<String>,
    email: Option<String>,
    phone: Option<String>,
    _mark: PhantomData<(Name, Email, Phone)>,
}



impl PersonBuilder<Missing, Missing, Missing> {
    fn new() -> Self {
        Self {
            name: None,
            email: None,
            phone: None,
            _mark: PhantomData,
        }
    }
}


impl<Email, Phone> PersonBuilder<Missing, Email, Phone> {
    fn name(self, name: impl Into<String>) -> PersonBuilder<Present, Email, Phone> {
        PersonBuilder {
            name: Some(name.into()),
            email: self.email,
            phone: self.phone,
            _mark: PhantomData,
        }
    }
}


impl<Name, Phone> PersonBuilder<Name, Missing, Phone> {
    fn email(self, email: impl Into<String>) -> PersonBuilder<Name, Present, Phone> {
        PersonBuilder {
            name: self.name,
            email: Some(email.into()),
            phone: self.phone,
            _mark: PhantomData,
        }
    }
}


impl<Name, Email> PersonBuilder<Name, Email, Missing> {
    fn phone(self, phone: impl Into<String>) -> PersonBuilder<Name, Email, Present> {
        PersonBuilder {
            name: self.name,
            email: self.email,
            phone: Some(phone.into()),
            _mark: PhantomData,
        }
    }
}


impl PersonBuilder<Present, Present, Present> {
    fn build(self) -> Person {
        Person {
            name: self.name.expect("unwrapped none in build method"),
            email: self.email.expect("unwrapped none in build method"),
            phone: self.phone.expect("unwrapped none in build method"),
        }
    }

}


fn test() {
    let builder = PersonBuilder::new();

    let builder = builder.name("Taisei");
    let builder = builder.email("test");
    let builder = builder.phone("test");

    builder.build();
}
