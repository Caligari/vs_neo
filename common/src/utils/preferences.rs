use std::collections::BTreeMap;

use num::clamp;


pub type PreferenceNumber = u16;
type PreferenceAnyNumber = i32;

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
pub enum PreferenceValue {
    Nothing,
    Boolean(bool),
    Number(PreferenceNumber),
    AnyNumber(PreferenceAnyNumber),
}

use PreferenceValue::*;


#[allow(dead_code)]
pub struct Preferences {
    preferences: BTreeMap<String, PreferenceValue>,
    filename: String,  // probably should be a path
}

impl Preferences {
    pub fn new ( save_file: &str ) -> Self {
        // TODO: make path in user/save_file.prefs
        // TODO: check path exists
        // TODO: then load prefs
        Preferences {
            preferences: BTreeMap::new(),
            filename: save_file.to_string(),
        }
    }

    pub fn get_preference ( &self, label: &str ) -> Option<PreferenceValue> {
        self.preferences.get(label).copied()
    }

    pub fn get_boolean_preference ( &self, label: &str ) -> Option<bool> {
        if let Some(Boolean(pref)) = self.preferences.get(label) {
            Some(*pref)
        } else { None }
    }

    pub fn get_number_preference ( &self, label: &str ) -> Option<PreferenceNumber> {
        if let Some(Number(pref)) = self.preferences.get(label) {
            Some(*pref)
        } else { None }
    }

    // was add_preferece
    pub fn set_preference ( &mut self, label: &str, value: PreferenceValue ) -> PreferenceValue {  // do we need a return here?
        self.preferences.insert(label.to_string(), value);
        value
    }

    pub fn set_boolean_preference ( &mut self, label: &str, value: bool ) -> bool {
        self.preferences.insert(label.to_string(), Boolean(value));
        value
    }

    pub fn set_number_preference ( &mut self, label: &str, value: PreferenceNumber ) -> PreferenceNumber {
        self.preferences.insert(label.to_string(), Number(value));
        value
    }

    /// This will convert a Number value to a Boolean if needed, or create a Boolean default
    pub fn constrain_boolean_preference ( &mut self, label: &str, default: bool ) -> bool {
        match self.preferences.get_mut(label) {
            Some(Boolean(pref)) => { *pref },
            Some(Number(pref)) => {
                let b = *pref != 0;
                self.set_preference(label, Boolean(b));
                b
            },
            Some(AnyNumber(pref)) => {
                let b = *pref != 0;
                self.set_preference(label, Boolean(b));
                b
            },

            None | Some(Nothing) => {
                self.set_preference(label, Boolean(default));
                default
            },
        }
    }

    /// This will convert an AnyNumber to a Number, or replace with the default, and constrain between min and max
    pub fn constrain_number_preference ( &mut self, label: &str, default: PreferenceNumber, min: PreferenceNumber, max: PreferenceNumber ) -> PreferenceNumber {
        // we ignore/overwrite Boolean values
        match self.preferences.get_mut(label) {
            Some(Number(pref)) => {
                *pref = clamp(*pref, min, max);
                *pref
            },

            Some(AnyNumber(pref)) => {
                let num = if let Ok(n) = PreferenceNumber::try_from(*pref) {
                    clamp(n, min, max)
                } else { clamp(default, min, max) };
                self.set_preference(label, Number(num));
                num
            },

            None | Some(Nothing) | Some(Boolean(..)) => {
                let num = clamp(default, min, max);
                self.set_preference(label, Number(num));
                num
            },
        }
    }
}