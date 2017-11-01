#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate libc;
mod sysdeezer;
mod errors;

use errors::{ DeezerError, ffi_error_to_enum };
use std::ffi::{ CString, CStr };

#[derive(Clone)]
pub struct DeezerAppConfig {
  anonymous_blob: Option<String>,
  user_profile_path: String,
  product_build_id: String,
  product_id: String,
  app_id: String
}

pub struct DeezerApp {
  connect: Option<*mut sysdeezer::dz_connect>,
  context: Option<*mut std::os::raw::c_void>,
  player: Option<*mut sysdeezer::dz_player>,

  config: DeezerAppConfig,
}

impl DeezerApp {
  pub fn new(config: DeezerAppConfig) -> DeezerApp {
    DeezerApp {
      connect: None,
      context: None,
      player: None,
      config
    }
  }

  pub fn connect(&mut self) -> Result<(), DeezerError> {
    let _user_profile_path = CString::new(self.config.user_profile_path.clone()).unwrap();
    let _product_build_id = CString::new(self.config.product_build_id.clone()).unwrap();
    let _product_id = CString::new(self.config.product_id.clone()).unwrap();
    let _app_id = CString::new(self.config.app_id.clone()).unwrap();

    let user_profile_path = _user_profile_path.as_ptr(); 
    let product_build_id = _product_build_id.as_ptr(); 
    let product_id = _product_id.as_ptr(); 
    let app_id = _app_id.as_ptr();

    unsafe {
      // Configuration transform
      let mut config: Box<_> = Box::new(sysdeezer::dz_connect_configuration {
        anonymous_blob: std::ptr::null_mut(),
        user_profile_path,
        product_build_id,
        product_id,
        app_id,

        app_has_crashed_delegate: None,
        connect_event_cb: None,
      });

      let connect_handler = sysdeezer::dz_connect_new(&mut *config);
      let context: *mut std::os::raw::c_void = std::ptr::null_mut();
      
      // Handle if unknow error of create connection handle
      if connect_handler.is_null() {
        return Err(DeezerError::Unknown);
      }

      // Get current device id
      let _device_id = sysdeezer::dz_connect_get_device_id(connect_handler);
      if let Ok(device_id) = CStr::from_ptr(_device_id).to_str() {
        println!("Device ID: {}", device_id);
      }

      // Setting cache path
      sysdeezer::dz_connect_cache_path_set(
        connect_handler, None,
        std::ptr::null_mut(),
        _user_profile_path.as_ptr()
      );

      // Activate session
      let connection_error = ffi_error_to_enum(sysdeezer::dz_connect_activate(connect_handler, context));
      match connection_error {
        DeezerError::NoError => {
          self.connect = Some(connect_handler);
          self.context = Some(context);
        },
        _ => { return Err(connection_error) }
      }

      Ok(())
    }
  }

  pub fn setup_player(&mut self) -> Result<(), DeezerError> {
    if let Some(connect) = self.connect.clone() {
      let context = self.context.clone().unwrap();

      unsafe {
        // Get player
        let player = sysdeezer::dz_player_new(connect);
        if player.is_null() { return Err(DeezerError::PlayerInitial) }

        // Activate player
        let activate_error = ffi_error_to_enum(sysdeezer::dz_player_activate(player, context));
        match activate_error {
          DeezerError::NoError => {},
          _ => { return Err(activate_error) }
        }

        //  Set Default volume
        let volume_error = ffi_error_to_enum(sysdeezer::dz_player_set_output_volume(
          player, None, std::ptr::null_mut(), 50
        ));

        match volume_error {
          DeezerError::NoError => {},
          _ => { return Err(volume_error) }
        }

        self.player = Some(player);
        Ok(())
      }
    } else {
      Err(DeezerError::PlayerInitial)
    }
  }

  pub fn set_access_token(&mut self, access_token: &str) -> Result<(), DeezerError> {
    let access_token = CString::new(access_token).unwrap();

    if let Some(connect) = self.connect.clone() {
      unsafe {
        let error = ffi_error_to_enum(sysdeezer::dz_connect_set_access_token(
          connect, None, std::ptr::null_mut(), access_token.as_ptr()
        ));

        match error {
          DeezerError::NoError => {
            println!("Token success set");
            Ok(())
          },
          _ => { return Err(error) }
        }
      }
    } else {
      Err(DeezerError::ConnectSessionLoginFailed)
    }
  }

  pub fn play_uri(&mut self, uri: &str) {
    let player = self.player.clone().unwrap();
    let uri = CString::new(uri).unwrap();

    unsafe {
      sysdeezer::dz_player_load(
        player, None, std::ptr::null_mut(), uri.as_ptr()  
      );

      sysdeezer::dz_player_play(
        player, None, std::ptr::null_mut(),
        sysdeezer::dz_player_play_command_t_DZ_PLAYER_PLAY_CMD_START_TRACKLIST,
        0
      );
    }
  }
}


fn main() {
  let access_token = "fr3hZZU32EKIJX74TEkTR9vgTtyu798fQfXRaMgzfTAigt3zAED";

  let config = DeezerAppConfig {
    user_profile_path: String::from("/home/anton/Deezer"), 
    product_id: String::from("DesktopCloudPlayer" ),
    product_build_id: String::from("1.0"),
    app_id: String::from("258282"),
    anonymous_blob: None
  };

  let mut app = DeezerApp::new(config.clone());
  
  if let Ok(_) = app.connect() {
    println!("Complete connect");
    
    if let Ok(_) = app.set_access_token(access_token) {
      println!("Complete set access token");

      if let Ok(_) = app.setup_player() {
        println!("Complete adding & activate player");

        // Play music
        app.play_uri("dzmedia:///track/10287076");
      }
    }
  }
}