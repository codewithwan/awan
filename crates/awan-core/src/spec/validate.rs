//! What makes a spec usable, as opposed to merely parseable.
//!
//! Separate from the spec's shape because they're two jobs: serde says whether
//! the TOML is a document, this says whether the document is a character. The
//! errors are the whole point — someone writing pixel art in a text file gets
//! the line they got wrong, not "invalid spec".

use super::*;

impl CharacterSpec {
    pub fn validate(&self) -> Result<(), SpecError> {
        let fail = |msg: String| Err(SpecError::Invalid(msg));

        if self.spec_version != SPEC_VERSION {
            return fail(format!(
                "spec_version {} is not supported (engine supports {SPEC_VERSION})",
                self.spec_version
            ));
        }
        let s = &self.sprite;
        for (set, name, len) in [
            (&s.rows, "rows", SPEC_H),
            (&s.sit_rows, "sit_rows", SPEC_H),
            (&s.leg_frames, "leg_frames", 4),
        ] {
            if set.len() != len {
                return fail(format!("sprite.{name} must have exactly {len} rows"));
            }
            for (i, row) in set.iter().enumerate() {
                if row.chars().count() != SPEC_W {
                    return fail(format!("sprite.{name}[{i}] must be {SPEC_W} pixels wide"));
                }
            }
        }
        if let Some(row) = &s.shimmer_row {
            if row.chars().count() != SPEC_W {
                return fail(format!("sprite.shimmer_row must be {SPEC_W} pixels wide"));
            }
        }
        // Face rows shift one row down when sitting, so they can't be last.
        for (idx, name, max) in [
            (s.eye_row, "eye_row", SPEC_H - 1),
            (s.mouth_row, "mouth_row", SPEC_H - 1),
            (s.legs_row, "legs_row", SPEC_H),
        ] {
            if idx >= max {
                return fail(format!("sprite.{name} {idx} out of range (max {max})"));
            }
        }
        if !s.rows[s.eye_row].contains('@') {
            return fail(format!(
                "sprite.rows[{}] (eye_row) must contain '@' eyes",
                s.eye_row
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn reference_spec_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../characters/awan.toml")
    }

    #[test]
    fn reference_character_loads_and_validates() {
        let spec = load(&reference_spec_path()).expect("characters/awan.toml must be valid");
        assert_eq!(spec.character.name, "Awan");
        assert_eq!(spec.sprite.rows.len(), 6);
        assert!(spec.character.palette.contains_key("body"));
    }

    #[test]
    fn version_mismatch_is_rejected() {
        let mut spec = load(&reference_spec_path()).unwrap();
        spec.spec_version = 999;
        assert!(matches!(spec.validate(), Err(SpecError::Invalid(_))));
    }

    #[test]
    fn eyeless_eye_row_is_rejected() {
        let mut spec = load(&reference_spec_path()).unwrap();
        spec.sprite.rows[spec.sprite.eye_row] = "##########".into();
        assert!(matches!(spec.validate(), Err(SpecError::Invalid(_))));
    }
}
