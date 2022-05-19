use std::fs;

pub fn read_definitions<T: for<'de> serde::Deserialize<'de>>(path: &str) -> Vec<T> {
	let mut type_definition_list = Vec::<T>::new();
	// Read directory for definition files
	for definition_result in fs::read_dir(&path).expect("Reading definition directory failed!") {
		// Get path of individual definition file
		let definition_path = match definition_result {
			Ok(val) => val.path(),
			Err(error) => panic!("Reading definition path {} failed! Error: {}", &path, error),
		};

		// Read definition file
		let type_definition = match fs::read(&definition_path) {
			Ok(val) => val,
			Err(error) => panic!(
				"Reading definition from path {:?} failed! Error: {}",
				&definition_path, error
			),
		};
		// Convert the definition to struct, and push to output vector
		type_definition_list.push(match serde_yaml::from_slice(&type_definition) {
			Ok(val) => val,
			Err(error) => panic!(
				"Converting definition from file {:?} from string to struct failed! Error: {}",
				&definition_path, error
			),
		});
	}
	return type_definition_list;
}
