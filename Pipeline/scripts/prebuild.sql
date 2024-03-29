-- find all versions having keword 'ffi', while keyword_id of 'ffi' is 178. One more condition: date before 2022-08-01

CREATE VIEW ffi_versions AS
SELECT
    crates.id as crate_id,
    versions.id as version_id,
    crates.name as crate_name,
    versions.num as version_num,
    versions.updated_at
FROM crates
    JOIN versions ON crates.id = versions.crate_id
    JOIN (
        SELECT
            DISTINCT crate_id
        FROM crates_keywords
        WHERE
            keyword_id = '178'
    ) AS keywords ON crates.id = keywords.crate_id
WHERE
	versions.updated_at <= '2022-08-01';