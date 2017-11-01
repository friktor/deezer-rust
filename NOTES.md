### Build
LD_PRELOAD=/home/anton/Projects/deezer-rust/libdeezer/libdeezer.so RUSTFLAGS="-C link-args=-L/home/anton/Projects/deezer-rust/libdeezer/" cargo run 

### Auth Deezer
_Get Code_
`https://connect.deezer.com/oauth/auth.php?app_id=258282&redirect_uri=https://tapok.me/deezer/redirect&perms=basic_access,email,manage_library,listening_history,delete_library`
_Get Token_
`https://connect.deezer.com/oauth/access_token.php?app_id=258282&secret=95a97a2983f5d00e99cb8a51425a6005&code=fr115673a96abba0b628e2e213a826d6&output=json`