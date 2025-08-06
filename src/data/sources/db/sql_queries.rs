///
/// Get all the channel items from the database
///
pub const SELECT_CHANNELS_QUERY: &str = "\
SELECT channels.id, channels.channel_id, channels.display_name, channels.icon \
FROM channels \
JOIN channel_packages ON channels.channel_id = channel_packages.channel_id \
WHERE channel_packages.package_id = $1 \
";

///
/// Insert a new channel into the database
///
pub const INSERT_CHANNEL_QUERY: &str = "INSERT INTO channels (channel_id, display_name, icon) VALUES ($1, $2, $3)";

///
/// Delete all channels from the database
///
pub const DELETE_CHANNELS_QUERY: &str = "DELETE FROM channels";

///
/// Insert a new package for a channel into the database
///
pub const INSERT_PACKAGE_QUERY: &str = "INSERT INTO channel_packages (channel_id, package_id) VALUES ($1, $2)";

///
/// Delete all packages from the database
///
pub const DELETE_PACKAGES_QUERY: &str = "DELETE FROM channel_packages";

///
/// Delete all programs from the database
///
pub const DELETE_PROGRAMS_QUERY: &str = "DELETE FROM programs";

///
/// Get all programs for a specific channel by channel_id
///
pub const FIND_PROGRAMS_BY_CHANNEL_ID_QUERY: &str = "\
SELECT * FROM programs WHERE channel_id = $1 \
AND start_time >= NOW() \
ORDER BY start_time ASC \
LIMIT 100
";

///
/// Get the current program for a specific channel by channel_id
///
pub const FIND_CURRENT_PROGRAM_BY_CHANNEL_ID_QUERY: &str = "\
SELECT * FROM programs \
WHERE channel_id = $1 \
AND start_time <= NOW() \
AND end_time >= NOW() \
LIMIT 1
";

pub const FIND_TONIGHT_PROGRAM_BY_CHANNEL_ID_QUERY: &str = "\
SELECT * FROM programs
WHERE channel_id = $1
AND start_time >= $2
-- duration is at least 30 minutes
AND (end_time - start_time) >= INTERVAL '30 minute'
ORDER BY start_time ASC
LIMIT 1
";