/*---------------------------------------------------------------------------------------------
 *  Crow Editor Bridge
 *  Intercepts high-level editor operations and forwards them to the Rust
 *  `crow-editor` / `crow-text` crates via Tauri IPC.
 *--------------------------------------------------------------------------------------------*/

import { invoke } from '../../../crow-bridge.js';

export class CrowEditorBridge {
	private static instance: CrowEditorBridge;

	static getInstance(): CrowEditorBridge {
		if (!CrowEditorBridge.instance) {
			CrowEditorBridge.instance = new CrowEditorBridge();
		}
		return CrowEditorBridge.instance;
	}

	// --- File operations (crow-text) ---

	async readFile(path: string): Promise<string> {
		return invoke<string>('read_file', { path });
	}

	async writeFile(path: string, content: string): Promise<void> {
		return invoke<void>('write_file', { path, content });
	}

	// --- Language detection (crow-syntax) ---

	async detectLanguage(filename: string): Promise<string> {
		try {
			return await invoke<string>('syntax_detect_language', { filename });
		} catch {
			return 'plaintext';
		}
	}

	// --- Git status (crow-git) ---

	async getGitStatus(repoRoot: string): Promise<any> {
		return invoke('git_status', { repoRoot });
	}

	// --- Search (crow-workspace) ---

	async searchInFiles(dir: string, query: string): Promise<any> {
		return invoke('search_text', { dir, query });
	}

	// --- Settings (crow-settings) ---

	async getSettings(section?: string): Promise<any> {
		return invoke('settings_get', { section: section ?? null });
	}

	// --- Theme (crow-theme) ---

	async getThemeList(): Promise<any[]> {
		return (await invoke<any[]>('theme_list')) ?? [];
	}

	async getThemeData(id: string): Promise<any> {
		return invoke('theme_get', { id });
	}
}
