#[cfg(target_os = "windows")]
const TARGET_PROCESS: &str = "LeagueClientUx.exe";
#[cfg(target_os = "linux")]
const TARGET_PROCESS: &str = "LeagueClientUx.";
#[cfg(target_os = "macos")]
const TARGET_PROCESS: &str = "LeagueClientUx";


#[derive(Debug, Clone)]
pub enum ProcessInfoError {
    ProcessNotAvailable,
    PortNotFound,
    AuthTokenNotFound,
}


// pub fn get_auth_info() -> Result<(String, String), AppError> {
//     let mut sys = System::new_all();
//     sys.refresh_processes();
//
//     let args = sys
//         .processes()
//         .values()
//         .find(|p| p.name() == TARGET_PROCESS)
//         .map(|p| p.cmd())
//         .ok_or(ProcessInfoError::ProcessNotAvailable)?;
//
//     let port = args
//         .iter()
//         .find(|arg| arg.starts_with("--app-port="))
//         .map(|arg| arg.strip_prefix("--app-port=").unwrap().to_string())
//         .ok_or(ProcessInfoError::PortNotFound)?;
//     let auth_token = args
//         .iter()
//         .find(|arg| arg.starts_with("--remoting-auth-token="))
//         .map(|arg| {
//             arg.strip_prefix("--remoting-auth-token=")
//                 .unwrap()
//                 .to_string()
//         })
//         .ok_or(ProcessInfoError::AuthTokenNotFound)?;
//
//     Ok((
//         general_purpose::STANDARD.encode(format!("riot:{}", auth_token)),
//         port,
//     ))
// }
//
//
//
// pub fn start_client(riot_path: &str) -> AppResult<()> {
//     let process = std::process::Command::new(format!("{}/LeagueClient.exe", riot_path))
//         .spawn()
//         .map_err(|e| format!("Failed to start client: {}", e)).unwrap();
//     // check if process started successfully
//     if process.id() == 0 {
//         return Err("Client failed to start".into());
//     }
//     Ok(())
//     // start leagueClientUx.exe
//     // check if it started successfully
//     // if it did, return Ok(())
//     // else return Err("Client failed to start")
// }