-- Add is_ai column to users table
ALTER TABLE users ADD COLUMN is_ai INTEGER NOT NULL DEFAULT 0;

-- Create index for efficient AI queries
CREATE INDEX IF NOT EXISTS idx_users_is_ai ON users(is_ai);

-- Insert 100 AI users with creative names and distributed Elo ratings
-- Rock-themed names (20)
INSERT INTO users (id, username, email, password_hash, elo, is_ai) VALUES
('ai-001', 'StoneWall', 'ai-001@bot.local', NULL, 985, 1),
('ai-002', 'RockSolid', 'ai-002@bot.local', NULL, 1015, 1),
('ai-003', 'GraniteGuard', 'ai-003@bot.local', NULL, 995, 1),
('ai-004', 'BoulderCrush', 'ai-004@bot.local', NULL, 1030, 1),
('ai-005', 'CragHammer', 'ai-005@bot.local', NULL, 970, 1),
('ai-006', 'MountainKing', 'ai-006@bot.local', NULL, 1050, 1),
('ai-007', 'PebbleSlinger', 'ai-007@bot.local', NULL, 960, 1),
('ai-008', 'RockGolem', 'ai-008@bot.local', NULL, 1080, 1),
('ai-009', 'StoneBreaker', 'ai-009@bot.local', NULL, 1005, 1),
('ai-010', 'CliffHanger', 'ai-010@bot.local', NULL, 990, 1),
('ai-011', 'BasaltBeast', 'ai-011@bot.local', NULL, 1025, 1),
('ai-012', 'QuartzQueen', 'ai-012@bot.local', NULL, 1040, 1),
('ai-013', 'RockSteady', 'ai-013@bot.local', NULL, 1010, 1),
('ai-014', 'FlintStrike', 'ai-014@bot.local', NULL, 975, 1),
('ai-015', 'MarbleMage', 'ai-015@bot.local', NULL, 1020, 1),
('ai-016', 'StoneFist', 'ai-016@bot.local', NULL, 1000, 1),
('ai-017', 'LavaRock', 'ai-017@bot.local', NULL, 1035, 1),
('ai-018', 'SlateSlammer', 'ai-018@bot.local', NULL, 980, 1),
('ai-019', 'ObsidianOgre', 'ai-019@bot.local', NULL, 1060, 1),
('ai-020', 'GravelGuru', 'ai-020@bot.local', NULL, 965, 1);

-- Paper-themed names (20)
INSERT INTO users (id, username, email, password_hash, elo, is_ai) VALUES
('ai-021', 'ScrollMaster', 'ai-021@bot.local', NULL, 1005, 1),
('ai-022', 'ParchmentPro', 'ai-022@bot.local', NULL, 1025, 1),
('ai-023', 'OrigamiKing', 'ai-023@bot.local', NULL, 995, 1),
('ai-024', 'PageTurner', 'ai-024@bot.local', NULL, 1010, 1),
('ai-025', 'InkWielder', 'ai-025@bot.local', NULL, 1045, 1),
('ai-026', 'PaperTrail', 'ai-026@bot.local', NULL, 970, 1),
('ai-027', 'FoldMaster', 'ai-027@bot.local', NULL, 1030, 1),
('ai-028', 'QuillStrike', 'ai-028@bot.local', NULL, 1000, 1),
('ai-029', 'ManuscriptMage', 'ai-029@bot.local', NULL, 1015, 1),
('ai-030', 'PaperCrane', 'ai-030@bot.local', NULL, 985, 1),
('ai-031', 'SheetSensei', 'ai-031@bot.local', NULL, 1050, 1),
('ai-032', 'NotebookNinja', 'ai-032@bot.local', NULL, 975, 1),
('ai-033', 'CardboardChamp', 'ai-033@bot.local', NULL, 1020, 1),
('ai-034', 'RollScroll', 'ai-034@bot.local', NULL, 990, 1),
('ai-035', 'OrigamiOwl', 'ai-035@bot.local', NULL, 1040, 1),
('ai-036', 'PaperPlane', 'ai-036@bot.local', NULL, 960, 1),
('ai-037', 'FoldedFury', 'ai-037@bot.local', NULL, 1055, 1),
('ai-038', 'InkBlot', 'ai-038@bot.local', NULL, 1005, 1),
('ai-039', 'ParchmentPower', 'ai-039@bot.local', NULL, 1035, 1),
('ai-040', 'CalligraphyKing', 'ai-040@bot.local', NULL, 980, 1);

-- Scissors-themed names (20)
INSERT INTO users (id, username, email, password_hash, elo, is_ai) VALUES
('ai-041', 'BladeRunner', 'ai-041@bot.local', NULL, 1010, 1),
('ai-042', 'CutAbove', 'ai-042@bot.local', NULL, 1025, 1),
('ai-043', 'SharpWit', 'ai-043@bot.local', NULL, 990, 1),
('ai-044', 'SnipMaster', 'ai-044@bot.local', NULL, 1040, 1),
('ai-045', 'EdgeLord', 'ai-045@bot.local', NULL, 1005, 1),
('ai-046', 'SliceSamurai', 'ai-046@bot.local', NULL, 1030, 1),
('ai-047', 'CutThroat', 'ai-047@bot.local', NULL, 975, 1),
('ai-048', 'ShearGenius', 'ai-048@bot.local', NULL, 1015, 1),
('ai-049', 'SnipSnap', 'ai-049@bot.local', NULL, 965, 1),
('ai-050', 'BladeStorm', 'ai-050@bot.local', NULL, 1050, 1),
('ai-051', 'CutterKing', 'ai-051@bot.local', NULL, 1000, 1),
('ai-052', 'SharpShooter', 'ai-052@bot.local', NULL, 995, 1),
('ai-053', 'TrimTitan', 'ai-053@bot.local', NULL, 1035, 1),
('ai-054', 'SliceNinja', 'ai-054@bot.local', NULL, 980, 1),
('ai-055', 'EdgeCrusher', 'ai-055@bot.local', NULL, 1020, 1),
('ai-056', 'SnipperSnap', 'ai-056@bot.local', NULL, 1045, 1),
('ai-057', 'CutMaster', 'ai-057@bot.local', NULL, 970, 1),
('ai-058', 'BladeWhisper', 'ai-058@bot.local', NULL, 1055, 1),
('ai-059', 'ShearForce', 'ai-059@bot.local', NULL, 1010, 1),
('ai-060', 'CutClutch', 'ai-060@bot.local', NULL, 1000, 1);

-- Speed-themed names (20)
INSERT INTO users (id, username, email, password_hash, elo, is_ai) VALUES
('ai-061', 'QuickDraw', 'ai-061@bot.local', NULL, 1015, 1),
('ai-062', 'SwiftStrike', 'ai-062@bot.local', NULL, 1005, 1),
('ai-063', 'LightningFast', 'ai-063@bot.local', NULL, 1040, 1),
('ai-064', 'RapidFire', 'ai-064@bot.local', NULL, 995, 1),
('ai-065', 'FlashBang', 'ai-065@bot.local', NULL, 1025, 1),
('ai-066', 'SpeedDemon', 'ai-066@bot.local', NULL, 980, 1),
('ai-067', 'BlitzKrieg', 'ai-067@bot.local', NULL, 1050, 1),
('ai-068', 'QuickSilver', 'ai-068@bot.local', NULL, 1010, 1),
('ai-069', 'FastForward', 'ai-069@bot.local', NULL, 970, 1),
('ai-070', 'VelocityViper', 'ai-070@bot.local', NULL, 1030, 1),
('ai-071', 'TurboThrower', 'ai-071@bot.local', NULL, 1000, 1),
('ai-072', 'SwiftShadow', 'ai-072@bot.local', NULL, 1035, 1),
('ai-073', 'RapidRush', 'ai-073@bot.local', NULL, 990, 1),
('ai-074', 'SpeedStar', 'ai-074@bot.local', NULL, 1020, 1),
('ai-075', 'FlashFreeze', 'ai-075@bot.local', NULL, 1005, 1),
('ai-076', 'QuickSnap', 'ai-076@bot.local', NULL, 1045, 1),
('ai-077', 'ZoomZapper', 'ai-077@bot.local', NULL, 965, 1),
('ai-078', 'FastFingers', 'ai-078@bot.local', NULL, 1055, 1),
('ai-079', 'SwiftStorm', 'ai-079@bot.local', NULL, 985, 1),
('ai-080', 'RapidReflexes', 'ai-080@bot.local', NULL, 1015, 1);

-- Power-themed names (20)
INSERT INTO users (id, username, email, password_hash, elo, is_ai) VALUES
('ai-081', 'ThunderFist', 'ai-081@bot.local', NULL, 1020, 1),
('ai-082', 'StormBringer', 'ai-082@bot.local', NULL, 1000, 1),
('ai-083', 'IronGrip', 'ai-083@bot.local', NULL, 1035, 1),
('ai-084', 'TitanForce', 'ai-084@bot.local', NULL, 1045, 1),
('ai-085', 'MegaSlam', 'ai-085@bot.local', NULL, 995, 1),
('ai-086', 'PowerSurge', 'ai-086@bot.local', NULL, 1010, 1),
('ai-087', 'ForceField', 'ai-087@bot.local', NULL, 1025, 1),
('ai-088', 'MightyMauler', 'ai-088@bot.local', NULL, 980, 1),
('ai-089', 'BruteForce', 'ai-089@bot.local', NULL, 1050, 1),
('ai-090', 'PowerPunch', 'ai-090@bot.local', NULL, 975, 1),
('ai-091', 'ThunderStrike', 'ai-091@bot.local', NULL, 1040, 1),
('ai-092', 'IronFist', 'ai-092@bot.local', NULL, 1005, 1),
('ai-093', 'MegaMight', 'ai-093@bot.local', NULL, 1030, 1),
('ai-094', 'PowerBlast', 'ai-094@bot.local', NULL, 990, 1),
('ai-095', 'ForceNova', 'ai-095@bot.local', NULL, 1015, 1),
('ai-096', 'TitanThrow', 'ai-096@bot.local', NULL, 1055, 1),
('ai-097', 'StormFury', 'ai-097@bot.local', NULL, 960, 1),
('ai-098', 'PowerHouse', 'ai-098@bot.local', NULL, 1020, 1),
('ai-099', 'MegaMash', 'ai-099@bot.local', NULL, 1000, 1),
('ai-100', 'ThunderBolt', 'ai-100@bot.local', NULL, 1010, 1);
