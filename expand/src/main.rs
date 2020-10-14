#[cfg(test)]
#[macro_use]
extern crate parameterized;
// or
//use parameterized::parameterized;
// optionally, you can rename the import (see below for an example), e.g.
//use parameterized::parameterized as pm;
fn main() {}

#[cfg_attr(not(test), allow(unused))]
enum WineRegion {
    Champagne,
    Jura,
    Languedoc,
    Loire(LoireArea),
}

#[cfg_attr(not(test), allow(unused))]
impl WineRegion {
    fn tasted(&self) -> Option<()> {
        match self {
            WineRegion::Champagne => Some(()),
            WineRegion::Jura => None,
            WineRegion::Languedoc => Some(()),
            WineRegion::Loire(area) => area.tasted(),
        }
    }
}

#[cfg_attr(not(test), allow(unused))]
enum LoireArea {
    Nantes,
    Touraine,
}

#[cfg_attr(not(test), allow(unused))]
impl LoireArea {
    fn tasted(&self) -> Option<()> {
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[parameterized(
        champagne = { WineRegion::Champagne },
        languedoc = { WineRegion::Languedoc },
        nantes    = { WineRegion::Loire(LoireArea::Nantes) },
        touraine  = { WineRegion::Loire(LoireArea::Touraine) },
    )]
    fn wines_tasted(region: WineRegion) {
        assert!(region.tasted().is_some())
    }

    #[parameterized(jura = {
        WineRegion::Jura,
    })]
    fn wines_not_tasted(region: WineRegion) {
        assert!(region.tasted().is_none())
    }
}
