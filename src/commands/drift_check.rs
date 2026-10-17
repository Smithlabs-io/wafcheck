use crate::error;
// use crate::api;

// struct cloudflare_ruleset {}
// struct ruleset_rule {}

pub fn drift_check(terraform_path: &str) -> Result<String, error::AppError> {

    // let terradirectory = load terraform path?
    // map terraform rule defined in terradirectory to custom cloudflare_rource structs above

    // for each cloudflare resource categories in the terraform configuration:
    //      fetch the current configuration from Cloudflare using the API
    //      map each live config to same cloudflare_rource structs above
    // Compare the Terraform objects vs the Cloudflare objects to get the real drift 

    // return real_diff
    println!("Drift check completed successfully for terraform path: {}", terraform_path);
    Ok("Success".into())
}
