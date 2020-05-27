use super::cmd::{OpenDialogOptions, SaveDialogOptions};
use crate::api::dialog::{pick_folder, save_file, select, select_multiple, Response};
use web_view::WebView;

fn map_response(response: Response) -> String {
  match response {
    Response::Okay(path) => format!(r#""{}""#, path).replace("\\", "\\\\"),
    Response::OkayMultiple(paths) => format!("{:?}", paths),
    Response::Cancel => panic!("unexpected response type"),
  }
}

pub fn open<T: 'static>(
  webview: &mut WebView<'_, T>,
  options: OpenDialogOptions,
  callback: String,
  error: String,
) {
  crate::execute_promise_sync(
    webview,
    move || {
      let response = if options.multiple {
        select_multiple(options.filter, options.default_path)
      } else if options.directory {
        pick_folder(options.default_path)
      } else {
        select(options.filter, options.default_path)
      };
      response.map(map_response).map_err(|e| e.into())
    },
    callback,
    error,
  );
}

pub fn save<T: 'static>(
  webview: &mut WebView<'_, T>,
  options: SaveDialogOptions,
  callback: String,
  error: String,
) {
  crate::execute_promise_sync(
    webview,
    move || {
      save_file(options.filter, options.default_path)
        .map(map_response)
        .map_err(|e| e.into())
    },
    callback,
    error,
  );
}
