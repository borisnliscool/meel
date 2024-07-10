ALTER TABLE mailing_list_subscribers
DROP CONSTRAINT mailing_list_subscribers_mailing_list_id_fkey,
ADD CONSTRAINT mailing_list_subscribers_mailing_list_id_fkey
    FOREIGN KEY (mailing_list_id) REFERENCES mailing_lists(id) ON DELETE CASCADE;