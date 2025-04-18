use serde::Serialize;

/// A structure representing a power zone.
///
/// Each power zone is represented by its name, and a power range represented
/// by a lower and upper power in Watts.
#[derive(Serialize, Debug, PartialEq)]
pub struct Zone {
    pub name: String,
    /// Lower end of the power zone
    pub lower: u32,
    /// Upper end of the power zone
    pub upper: u32,
}

/// Calculate power zones based on an Functional Threshold Power (FTP) value.
///
/// # Example
///
/// ```
/// use powerzones::calc_power_zones;
/// let ftp: u32 = 200;
/// let zones = calc_power_zones(ftp);
/// ```
pub fn calc_power_zones(ftp: u32) -> Vec<Zone> {
    let mut zones: Vec<Zone> = Vec::new();
    let mut lower = 0;
    for (name, upper_percent) in vec![
        ("Recovery", 0.55), ("Endurance", 0.75), ("Tempo", 0.9),
        ("Threshold", 1.05), ("VO2Max", 1.2), ("Anaerobic Capacity", 1.5)] {
        let upper = watts(ftp, upper_percent);
        zones.push(Zone{name: name.to_string(), lower, upper});
        lower = upper+1;
    }
    zones
}

fn watts(ftp: u32, target: f32) -> u32 {
    ((ftp as f32) * target).round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zone1() {
        let zones = calc_power_zones(189);
        assert_eq!(zones[0], Zone{name: "Recovery".to_owned(), lower: 0, upper: 104});
        assert_eq!(zones[1], Zone{name: "Endurance".to_owned(), lower: 105, upper: 142});
        assert_eq!(zones[2], Zone{name: "Tempo".to_owned(), lower: 143, upper: 170});
        assert_eq!(zones[3], Zone{name: "Threshold".to_owned(), lower: 171, upper: 198});
        assert_eq!(zones[4], Zone{name: "VO2Max".to_owned(), lower: 199, upper: 227});
        assert_eq!(zones[5], Zone{name: "Anaerobic Capacity".to_owned(), lower: 228, upper: 284});
    }
}
