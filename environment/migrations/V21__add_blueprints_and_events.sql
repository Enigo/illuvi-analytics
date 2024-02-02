alter table collection drop column enabled;

UPDATE asset
SET attribute = CASE
                    WHEN token_address = '0x9e0d99b864e1ac12565125c5a82b59adea5a09cd' THEN concat('Tier ', metadata->>'tier')
                    WHEN token_address = '0xc1f1da534e227489d617cd742481fd5a23f6a003' THEN concat(metadata->>'name',
                                                                                                  CASE
                                                                                                      WHEN (metadata->>'Alpha')::boolean THEN ' Alpha'
                                                                                                      END, ' Wave ', metadata->>'Wave')
                    WHEN token_address = '0x8cceea8cfb0f8670f4de3a6cd2152925605d19a8' THEN concat('Set ', metadata->>'Set', ' Wave ', metadata->>'Wave', ' Tier ', metadata->>'Tier',
                                                                                                  CASE
                                                                                                      WHEN (metadata->>'Finish' = 'Holo') THEN ' Holo'
                                                                                                      END)
                    WHEN token_address = '0x844a2a2b4c139815c1da4bdd447ab558bb9a7d24' THEN concat('Set ', metadata->>'Set', ' Tier ', metadata->>'Tier',
                                                                                                  ' Stage ', metadata->>'Stage')
                    WHEN token_address = '0x07fb805d026194d188014fc7303e69f412eb7cb1' THEN concat('Tier ', metadata->>'Item Tier', ' Stage ', metadata->>'Item Stage')
                    WHEN token_address = '0x0d78b8aeddb8d3c8b8903a474f8a91855bfdf6f2' THEN concat(metadata->>'Promotion',
                                                                                                  CASE
                                                                                                      WHEN (metadata->>'Finish' = 'Holo') THEN ' Holo'
                                                                                                      END)
    END;

CREATE OR REPLACE FUNCTION update_asset_attribute()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.attribute = CASE
                        WHEN NEW.token_address = '0x9e0d99b864e1ac12565125c5a82b59adea5a09cd' THEN concat('Tier ', NEW.metadata->>'tier')
                        WHEN NEW.token_address = '0xc1f1da534e227489d617cd742481fd5a23f6a003' THEN concat(NEW.metadata->>'name',
                                                                                                          CASE
                                                                                                              WHEN (NEW.metadata->>'Alpha')::boolean THEN ' Alpha'
                                                                                                              END, ' Wave ', NEW.metadata->>'Wave')
                        WHEN NEW.token_address = '0x8cceea8cfb0f8670f4de3a6cd2152925605d19a8' THEN concat('Set ', NEW.metadata->>'Set', ' Wave ', NEW.metadata->>'Wave', ' Tier ', NEW.metadata->>'Tier',
                                                                                                          CASE
                                                                                                              WHEN (NEW.metadata->>'Finish' = 'Holo') THEN ' Holo'
                                                                                                              END)
                        WHEN NEW.token_address = '0x844a2a2b4c139815c1da4bdd447ab558bb9a7d24' THEN concat('Set ', NEW.metadata->>'Set', ' Tier ', NEW.metadata->>'Tier',
                                                                                                          ' Stage ', NEW.metadata->>'Stage')
                        WHEN NEW.token_address = '0x07fb805d026194d188014fc7303e69f412eb7cb1' THEN concat('Tier ', NEW.metadata->>'Item Tier', ' Stage ', NEW.metadata->>'Item Stage')
                        WHEN NEW.token_address = '0x0d78b8aeddb8d3c8b8903a474f8a91855bfdf6f2' THEN concat(NEW.metadata->>'Promotion',
                                                                                                          CASE
                                                                                                              WHEN (NEW.metadata->>'Finish' = 'Holo') THEN ' Holo'
                                                                                                              END)
        END;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
