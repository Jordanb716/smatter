use std::fs;

/// Reads the files in a directory filled with YAML templates, and returns a vector of deserialized structs made from their contents.
pub fn read_definitions<T: for<'de> serde::Deserialize<'de>>(path: &str) -> Vec<T> {
	let mut definition_list = Vec::<T>::new();
	// Read directory for definition files
	for definition_result in fs::read_dir(path).expect("Reading definition directory failed!") {
		// Get path of individual definition file
		let definition_path = match definition_result {
			Ok(val) => {
				if val.file_name() == "template.yaml" {
					break; // Skip templates
				} else {
					val.path()
				}
			}
			Err(error) => panic!("Reading definition path {} failed! Error: {}", path, error),
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
		definition_list.push(match serde_yaml::from_slice(&type_definition) {
			Ok(val) => val,
			Err(error) => panic!(
				"Converting definition from file {:?} from string to struct failed! Error: {}",
				&definition_path, error
			),
		});
	}
	return definition_list;
}

/// Takes an example of a struct (```definition_template```) and writes it out to ```path``` as a template
/// to help with editing item definitions.
pub fn write_definition_template<T: serde::Serialize>(path: &str, definition_template: T) {
	// Convert template struct to a yaml string
	let definition_template = match serde_yaml::to_string(&definition_template) {
		Ok(val) => val,
		Err(error) => panic!(
			"Template serialization failed for path {}. Error: {}",
			path, error
		),
	};
	// Try to create the directory in case it hasn't been initialized
	match fs::create_dir_all(path) {
		Ok(_) => (),
		Err(error) => panic!("Failed to create path {}. Error: {}", path, error),
	};
	// Write the template out
	match fs::write(path.to_string() + "template.yaml", definition_template) {
		Ok(_) => (),
		Err(error) => panic!("Writing Template at {} failed. Error: {}", path, error),
	};
}
