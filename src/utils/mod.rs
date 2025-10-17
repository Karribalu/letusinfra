pub mod constants;

// fn plan_components(config: &InfraConfig) {
//     let dependency_tree = plan_components_sequence(&config.components);

//     for component in &config.components {
//         match component.component_type.as_str() {
//             "EC2Instance" => {
//                 // Create EC2 instance Terraform code
//                 plan_ec2_instance(&config.region, component);
//             }
//             _ => {
//                 eprintln!("Unsupported component type: {}", component.component_type);
//             }
//         }
//     }
// }

// fn plan_components_sequence(
//     components: &[crate::models::Component],
// ) -> Vec<&crate::models::Component> {
//     let mut sequence = Vec::new();
//     let mut visited = HashSet::new();

//     for component in components {
//         if !visited.contains(component) {
//             plan_component_sequence(component, &mut sequence, &mut visited);
//         }
//     }

//     sequence
// }
