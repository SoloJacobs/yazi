use anyhow::anyhow;
use yazi_plugin::utils::PreviewLock;
use yazi_shared::event::Exec;

use crate::tab::Tab;

pub struct Opt {
	lock: PreviewLock,
}

impl TryFrom<&Exec> for Opt {
	type Error = anyhow::Error;

	fn try_from(e: &Exec) -> Result<Self, Self::Error> {
		Ok(Self { lock: e.take_data().ok_or_else(|| anyhow!("invalid data"))? })
	}
}

impl Tab {
	pub fn preview(&mut self, opt: impl TryInto<Opt>) -> bool {
		let Some(hovered) = self.current.hovered().map(|h| &h.url) else {
			return self.preview.reset();
		};

		let Ok(opt) = opt.try_into() else {
			return false;
		};

		if hovered != &opt.lock.url {
			return false;
		}

		self.preview.lock = Some(opt.lock);
		true
	}
}
