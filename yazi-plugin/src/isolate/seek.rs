use mlua::TableExt;
use yazi_config::LAYOUT;
use yazi_shared::{emit, event::Exec, Layer};

use crate::{bindings::{Cast, File}, elements::Rect, OptData, LUA};

pub fn seek_sync(exec: &Exec, file: yazi_shared::fs::File, units: i16) {
	let data = OptData {
		args: vec![],
		cb:   Some(Box::new(move |plugin| {
			plugin.set("file", File::cast(&LUA, file)?)?;
			plugin.set("area", Rect::cast(&LUA, LAYOUT.load().preview)?)?;
			plugin.call_method("seek", units)
		})),
		tx:   None,
	};
	emit!(Call(
		Exec::call("plugin", vec![exec.cmd.to_owned()]).with_bool("sync", true).with_data(data).vec(),
		Layer::App
	));
}
