pub const CREATE_SCHEMA: &str = r"
    CREATE TABLE IF NOT EXISTS roots(
        id INTEGER PRIMARY KEY,
        path TEXT UNIQUE NOT NULL
    );

    CREATE TABLE IF NOT EXISTS songs(
        id BLOB PRIMARY KEY,
        title TEXT NOT NULL,
        year INTEGER,
        path TEXT UNIQUE NOT NULL,
        artist_id INTEGER,
        album_id INTEGER,
        track_no INTEGER,
        disc_no INTEGER,
        duration REAL,
        channels INTEGER,
        bit_rate INTEGER,
        sample_rate INTEGER,
        format INTEGER,
        FOREIGN KEY(artist_id) REFERENCES artists(id),
        FOREIGN KEY(album_id) REFERENCES albums(id)
    );

    CREATE TABLE IF NOT EXISTS artists(
        id INTEGER PRIMARY KEY,
        name TEXT UNIQUE NOT NULL
    );

    CREATE TABLE IF NOT EXISTS albums(
        id INTEGER PRIMARY KEY,
        title TEXT NOT NULL,
        artist_id INTEGER,
        FOREIGN KEY(artist_id) REFERENCES artists(id),
        UNIQUE (title, artist_id)
    );

    CREATE TABLE IF NOT EXISTS waveforms(
        song_id BLOB PRIMARY KEY,
        waveform BLOB,
        FOREIGN KEY(song_id) REFERENCES songs(id) ON DELETE CASCADE
    );

    CREATE TABLE IF NOT EXISTS history(
        id INTEGER PRIMARY KEY,
        song_id BLOB NOT NULL,
        timestamp INTEGER NOT NULL,
        FOREIGN KEY(song_id) REFERENCES songs(id) ON DELETE CASCADE
    );

    CREATE TABLE IF NOT EXISTS plays(
        song_id BLOB PRIMARY KEY,
        count INTEGER,
        FOREIGN KEY(song_id) REFERENCES songs(id) ON DELETE CASCADE
    );

    CREATE TABLE IF NOT EXISTS session_state(
        key TEXT PRIMARY KEY,
        value TEXT NOT NULL
    );

    CREATE TABLE IF NOT EXISTS playlists(
        id INTEGER PRIMARY KEY,
        name TEXT UNIQUE NOT NULL,
        updated_at INTEGER NOT NULL
    );

    CREATE TABLE IF NOT EXISTS playlist_songs(
        id INTEGER PRIMARY KEY,
        song_id BLOB NOT NULL,
        playlist_id INTEGER NOT NULL,
        position INTEGER NOT NULL,
        FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE,
        FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
        UNIQUE(playlist_id, position)
    );

    CREATE TABLE IF NOT EXISTS scan_cache(
        key TEXT PRIMARY KEY,
        value BLOB NOT NULL
    );

    CREATE TABLE IF NOT EXISTS now_playing(
        id INTEGER PRIMARY KEY CHECK(id = 1),
        song_id BLOB NOT NULL,
        position_secs REAL NOT NULL DEFAULT 0,
        started_at INTEGER NOT NULL,
        FOREIGN KEY(song_id) REFERENCES songs(id) ON DELETE CASCADE
    );

    CREATE VIEW IF NOT EXISTS now_playing_v1 AS
    SELECT
      s.title           AS title,
      ar.name           AS artist,
      al.title          AS album,
      s.track_no        AS track_no,
      s.disc_no         AS disc_no,
      s.year            AS year,
      s.path            AS path,
      s.duration        AS duration_secs,
      np.position_secs  AS position_secs,
      np.started_at     AS started_at
    FROM now_playing np
    JOIN songs   s  ON s.id       = np.song_id
    LEFT JOIN artists ar ON ar.id = s.artist_id
    LEFT JOIN albums  al ON al.id = s.album_id
    WHERE np.id = 1;
";
