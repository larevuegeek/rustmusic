import Database from '@tauri-apps/plugin-sql';

let dbInstance: Database | null = null;

export async function getDB(): Promise<Database>  {
  if (!dbInstance) {
    dbInstance = await Database.load("sqlite:rustmusic.db");
  }
  return dbInstance;
}