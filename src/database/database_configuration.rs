use crate::configuration::get_configuration;

pub enum DBCollectionSettings {
    Users,
    Exercises,
    TrainingPlans,
}

impl DBCollectionSettings {
    pub fn value(&self) -> &str {
        match *self {
            DBCollectionSettings::Users => "users",
            DBCollectionSettings::Exercises => "exercises",
            DBCollectionSettings::TrainingPlans => "training_plans",
        }
    }
}

pub struct DBSettings {
    pub name: String,
}

impl Default for DBSettings {
    fn default() -> Self {
        DBSettings {
            name: get_database_name(),
        }
    }
}

fn get_database_name() -> String {
    let configuration = get_configuration().expect("Failed to retrieve the configuration file");
    configuration.database.database_name.to_string()
}
