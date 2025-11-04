Password: superadmin123 -> Hash: $2b$12$1MNfH3KR0knWJBQk.YZ5MeXANUmvzGvj52j2RkroK8s/Tu2Dns7VG
Password: admin123 -> Hash: $2b$12$PxvAb1.BKewkD5lgGThncueBC8V33Y7NWnTkFUHWtgHp47a7cClO.
Password: operator123 -> Hash: $2b$12$u5P2fCYeHvzCE0AE2Y5AIePKQiS7aSpjF87s1sjVPAHeRwjGbeOmu
Password: viewer123 -> Hash: $2b$12$QJUTmtIm9Ur/p5HoNoTDfuH1QvpbBWfCVhgGlZtcw9xtYUOT1fDCC




UPDATE users SET password_hash = '$2b$12$1MNfH3KR0knWJBQk.YZ5MeXANUmvzGvj52j2RkroK8s/Tu2Dns7VG' WHERE username = 'superadmin';
UPDATE users SET password_hash = '$2b$12$PxvAb1.BKewkD5lgGThncueBC8V33Y7NWnTkFUHWtgHp47a7cClO.' WHERE username = 'admin';
UPDATE users SET password_hash = '$2b$12$u5P2fCYeHvzCE0AE2Y5AIePKQiS7aSpjF87s1sjVPAHeRwjGbeOmu' WHERE username = 'operator1';
UPDATE users SET password_hash = '$2b$12$QJUTmtIm9Ur/p5HoNoTDfuH1QvpbBWfCVhgGlZtcw9xtYUOT1fDCC' WHERE username = 'viewer1';