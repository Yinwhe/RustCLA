-- get version info in abicheck status
CREATE VIEW
    abicheck_status_with_info AS
SELECT
    crate_name,
    version_num,
    status_details,
    updated_at
FROM (
        SELECT *
        FROM abicheck_status
        ORDER BY
            status_details ASC,
            version_id DESC
    ) AS tmp
    JOIN ffi_versions ON tmp.version_id = ffi_versions.version_id
ORDER BY
    tmp.status_details ASC,
    tmp.version_id DESC