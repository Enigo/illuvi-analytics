CREATE INDEX idx_asset_metadata_base_illuvitar_token_id
    ON asset (((metadata->>'Base Illuvitar Token Id')::integer));
CREATE INDEX idx_asset_metadata_source_disk_id
    ON asset (((metadata->>'Source Disk Id')::integer));
CREATE INDEX idx_asset_token_address
    ON asset (token_address);