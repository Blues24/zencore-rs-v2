use crate::common::{CommonThemes, Colors, Symbols, Emphasis};

pub struct ThemeResolver{
    default: CommonThemes,
    user: Option<CommonThemes>,
    OverrideThemes: Option<CommonThemes>,
}

impl ThemeResolver {
    pub fn new(default: CommonThemes) -> Self {
        Self {
            default,
            user: None,
            OverrideThemes: None,
        }
    }

    pub fn apply_user_theme(mut self, theme: CommonThemes) -> Self {
        self.user = Some(theme);
        self 
    }

    pub fn OverrideThemes_current_theme(mut self, theme: CommonThemes) -> Self {
        self.OverrideThemes = Some(theme);
        self 
    }

    pub fn resolv_theme(self) -> CommonThemes {
        let mut result = self.default;

        if let Some(user) = self.user {
            result = self::merge_common_theme(result, user);
        }

        if let Some(_ovr) = self.OverrideThemes {
            result = self::merge_common_theme(result, _ovr);
        }

        result 

    }
}    

fn merge_common_theme(_base: CommonThemes, _ovr: CommonThemes) -> CommonThemes {
        CommonThemes {
            colors: self::merge_colors(_base.colors, _ovr.colors),
            symbols: self::merge_symbols(_base.symbols, _ovr.symbols),
            emphasis: self::merge_emphasis(_base.emphasis, _ovr.emphasis),
        }
}

 
fn merge_colors(_base: Colors, _over: Colors) -> Colors {
        Colors {
            primary: _over.primary,
            secondary: _over.secondary,
            success: _over.success,
            warning: _over.warning,
            error: _over.error,
            muted: _over.muted,
        }
}


fn merge_symbols(_base: Symbols, _over: Symbols) -> Symbols {
        _over 
 }


fn merge_emphasis(_base: Emphasis, _over: Emphasis) -> Emphasis {
        _over 
}


