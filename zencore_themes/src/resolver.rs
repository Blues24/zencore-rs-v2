use crate::common::{CommonTheme, Colors, Symbols, Emphasis};

pub struct ThemeResolver{
    default: CommonTheme,
    user: Option<CommonTheme>
    _override: Option<CommonTheme>
}

impl ThemeResolver {
    pub fn new(default: CommonTheme) -> Self {
        Self {
            default,
            user: None,
            _override: None,
        }
    }

    pub fn apply_user_theme(mut self, theme: CommonTheme) -> Self {
        self.user = Some(theme);
        self 
    }

    pub fn override_current_theme(mut self, theme: CommonTheme) -> Self {
        self._override = Some(theme);
        self 
    }

    pub fn resolv_theme(self) -> CommonTheme {
        let mut result = self.default;

        if let Some(user) = self.user {
            result = merge_common_theme(result, user);
        }

        if let Some(_ovr) = self._override {
            result = merge_common_theme(result, _ovr);
        }

        result 

    }
    
    fn merge_common_theme(base: CommonTheme, _ovr: CommonTheme) -> CommonTheme {
        CommonTheme {
            colors: merge_colors(base.colors, _ovr.colors),
            symbols: merge_symbols(base.symbols, _ovr.symbols),
            emphasis: merge_emphasis(base.emphasis, _ovr.emphasis),
        }
    }

    fn merge_colors(base: Colors, _over: Colors) -> Colors {
        Colors {
            primary: _over.primary,
            secondary: _over.primary,
            success: _over.success,
            warning: _over.warning,
            error: _over.error,
            muted: _over.muted,
        }
    }

    fn merge_symbols(base: Symbols, _over: Symbols) -> Symbols {
        _over 
    }

    fn merge_emphasis(base: Emphasis, _over: Emphasis) -> Emphasis {
        _over 
    }

}
