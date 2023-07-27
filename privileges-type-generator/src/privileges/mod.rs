#[derive(Debug)]
pub enum Posts {
	Create,
	Read,
	Update,
	Delete,
}
impl std::str::FromStr for Posts {
	type Err = ();
	fn from_str(input: &str) -> Result<Self, Self::Err> {
		match input {
			"create" => Ok(Posts::Create),
			"read" => Ok(Posts::Read),
			"update" => Ok(Posts::Update),
			"delete" => Ok(Posts::Delete),
			_ => Err(()),
		}
	}
}
#[derive(Debug)]
pub enum Robots {
	Pilot,
	Create,
	Read,
	Update,
	Delete,
}
impl std::str::FromStr for Robots {
	type Err = ();
	fn from_str(input: &str) -> Result<Self, Self::Err> {
		match input {
			"pilot" => Ok(Robots::Pilot),
			"create" => Ok(Robots::Create),
			"read" => Ok(Robots::Read),
			"update" => Ok(Robots::Update),
			"delete" => Ok(Robots::Delete),
			_ => Err(()),
		}
	}
}
#[derive(Debug)]
pub enum Users {
	Create,
	Read,
	Update,
	Delete,
}
impl std::str::FromStr for Users {
	type Err = ();
	fn from_str(input: &str) -> Result<Self, Self::Err> {
		match input {
			"create" => Ok(Users::Create),
			"read" => Ok(Users::Read),
			"update" => Ok(Users::Update),
			"delete" => Ok(Users::Delete),
			_ => Err(()),
		}
	}
}
