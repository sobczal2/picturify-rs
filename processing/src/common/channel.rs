#[derive(Copy, Clone)]
pub enum ChannelSelector {
    Rgba(RgbaChannelSelector),
    Hsva(HsvaChannelSelector),
    Hsla(HslaChannelSelector),
    La(LaChannelSelector),
}

impl ChannelSelector {
    pub fn red_enabled(&self) -> bool {
        match self {
            ChannelSelector::Rgba(selector) => selector.red_enabled(),
            ChannelSelector::Hsva(_) => false,
            ChannelSelector::Hsla(_) => false,
            ChannelSelector::La(_) => false,
        }
    }

    pub fn green_enabled(&self) -> bool {
        match self {
            ChannelSelector::Rgba(selector) => selector.green_enabled(),
            ChannelSelector::Hsva(_) => false,
            ChannelSelector::Hsla(_) => false,
            ChannelSelector::La(_) => false,
        }
    }

    pub fn blue_enabled(&self) -> bool {
        match self {
            ChannelSelector::Rgba(selector) => selector.blue_enabled(),
            ChannelSelector::Hsva(_) => false,
            ChannelSelector::Hsla(_) => false,
            ChannelSelector::La(_) => false,
        }
    }

    pub fn alpha_enabled(&self) -> bool {
        match self {
            ChannelSelector::Rgba(selector) => selector.alpha_enabled(),
            ChannelSelector::Hsva(_) => false,
            ChannelSelector::Hsla(_) => false,
            ChannelSelector::La(_) => false,
        }
    }

    pub fn hue_enabled(&self) -> bool {
        match self {
            ChannelSelector::Rgba(_) => false,
            ChannelSelector::Hsva(selector) => selector.hue_enabled(),
            ChannelSelector::Hsla(selector) => selector.hue_enabled(),
            ChannelSelector::La(_) => false,
        }
    }

    pub fn saturation_enabled(&self) -> bool {
        match self {
            ChannelSelector::Rgba(_) => false,
            ChannelSelector::Hsva(selector) => selector.saturation_enabled(),
            ChannelSelector::Hsla(selector) => selector.saturation_enabled(),
            ChannelSelector::La(_) => false,
        }
    }

    pub fn value_enabled(&self) -> bool {
        match self {
            ChannelSelector::Rgba(_) => false,
            ChannelSelector::Hsva(selector) => selector.value_enabled(),
            ChannelSelector::Hsla(_) => false,
            ChannelSelector::La(_) => false,
        }
    }

    pub fn lightness_enabled(&self) -> bool {
        match self {
            ChannelSelector::Rgba(_) => false,
            ChannelSelector::Hsva(_) => false,
            ChannelSelector::Hsla(selector) => selector.lightness_enabled(),
            ChannelSelector::La(selector) => selector.lightness_enabled(),
        }
    }
}
#[derive(Copy, Clone)]
pub struct RgbaChannelSelector {
    red: bool,
    green: bool,
    blue: bool,
    alpha: bool,
}

impl RgbaChannelSelector {
    pub fn new(red: bool, green: bool, blue: bool, alpha: bool) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn red_enabled(&self) -> bool {
        self.red
    }

    pub fn green_enabled(&self) -> bool {
        self.green
    }

    pub fn blue_enabled(&self) -> bool {
        self.blue
    }

    pub fn alpha_enabled(&self) -> bool {
        self.alpha
    }
}

impl Default for RgbaChannelSelector {
    fn default() -> Self {
        Self {
            red: true,
            green: true,
            blue: true,
            alpha: false,
        }
    }
}

#[derive(Copy, Clone)]
pub struct HsvaChannelSelector {
    hue: bool,
    saturation: bool,
    value: bool,
    alpha: bool,
}

impl HsvaChannelSelector {
    pub fn new(hue: bool, saturation: bool, value: bool, alpha: bool) -> Self {
        Self {
            hue,
            saturation,
            value,
            alpha,
        }
    }

    pub fn hue_enabled(&self) -> bool {
        self.hue
    }

    pub fn saturation_enabled(&self) -> bool {
        self.saturation
    }

    pub fn value_enabled(&self) -> bool {
        self.value
    }

    pub fn alpha_enabled(&self) -> bool {
        self.alpha
    }
}

impl Default for HsvaChannelSelector {
    fn default() -> Self {
        Self {
            hue: true,
            saturation: true,
            value: true,
            alpha: false,
        }
    }
}

#[derive(Copy, Clone)]
pub struct HslaChannelSelector {
    hue: bool,
    saturation: bool,
    lightness: bool,
    alpha: bool,
}

impl HslaChannelSelector {
    pub fn new(hue: bool, saturation: bool, lightness: bool, alpha: bool) -> Self {
        Self {
            hue,
            saturation,
            lightness,
            alpha,
        }
    }

    pub fn hue_enabled(&self) -> bool {
        self.hue
    }

    pub fn saturation_enabled(&self) -> bool {
        self.saturation
    }

    pub fn lightness_enabled(&self) -> bool {
        self.lightness
    }

    pub fn alpha_enabled(&self) -> bool {
        self.alpha
    }
}

impl Default for HslaChannelSelector {
    fn default() -> Self {
        Self {
            hue: true,
            saturation: true,
            lightness: true,
            alpha: false,
        }
    }
}

#[derive(Copy, Clone)]
pub struct LaChannelSelector {
    lightness: bool,
    alpha: bool,
}

impl LaChannelSelector {
    pub fn new(lightness: bool, alpha: bool) -> Self {
        Self { lightness, alpha }
    }

    pub fn lightness_enabled(&self) -> bool {
        self.lightness
    }

    pub fn alpha_enabled(&self) -> bool {
        self.alpha
    }
}

impl Default for LaChannelSelector {
    fn default() -> Self {
        Self {
            lightness: true,
            alpha: false,
        }
    }
}
