//! 設定モジュール
//!
//! 拡張子とカテゴリのマッピングを定義します。
//! 将来的に外部設定ファイル（TOML/JSON）から読み込む形に拡張可能な設計です。

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fmt;

/// ファイルカテゴリの列挙型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    Images,
    Videos,
    Documents,
    Music,
    Archives,
    Code,
    Others,
}

impl Category {
    /// カテゴリ名をフォルダ名として取得
    pub fn folder_name(&self) -> &'static str {
        match self {
            Category::Images => "Images",
            Category::Videos => "Videos",
            Category::Documents => "Documents",
            Category::Music => "Music",
            Category::Archives => "Archives",
            Category::Code => "Code",
            Category::Others => "Others",
        }
    }

    /// 全カテゴリのリストを取得
    pub fn all() -> &'static [Category] {
        &[
            Category::Images,
            Category::Videos,
            Category::Documents,
            Category::Music,
            Category::Archives,
            Category::Code,
            Category::Others,
        ]
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.folder_name())
    }
}

/// 拡張子からカテゴリへのマッピング
///
/// 小文字の拡張子をキーとして、対応するカテゴリを値として持つHashMap。
/// `once_cell::sync::Lazy` により、初回アクセス時に一度だけ初期化されます。
pub static EXTENSION_MAP: Lazy<HashMap<&'static str, Category>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Images - 画像ファイル
    let image_extensions = [
        "jpg", "jpeg", "png", "gif", "bmp", "svg", "webp", "ico", "tiff", "tif", "heic", "heif",
        "raw", "cr2", "nef", "arw", "dng", "psd", "ai", "eps",
    ];
    for ext in image_extensions {
        map.insert(ext, Category::Images);
    }

    // Videos - 動画ファイル
    let video_extensions = [
        "mp4", "avi", "mkv", "mov", "wmv", "flv", "webm", "m4v", "mpeg", "mpg", "3gp", "3g2",
        "vob", "ogv", "mts", "m2ts", "ts",
    ];
    for ext in video_extensions {
        map.insert(ext, Category::Videos);
    }

    // Documents - ドキュメントファイル
    let document_extensions = [
        "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "rtf", "odt", "ods", "odp",
        "csv", "pages", "numbers", "key", "epub", "mobi", "djvu", "xps",
    ];
    for ext in document_extensions {
        map.insert(ext, Category::Documents);
    }

    // Music - 音楽ファイル
    let music_extensions = [
        "mp3", "wav", "flac", "aac", "ogg", "wma", "m4a", "aiff", "aif", "ape", "alac", "opus",
        "mid", "midi",
    ];
    for ext in music_extensions {
        map.insert(ext, Category::Music);
    }

    // Archives - アーカイブファイル
    let archive_extensions = [
        "zip", "rar", "7z", "tar", "gz", "bz2", "xz", "dmg", "iso", "tgz", "tbz2", "txz", "cab",
        "lzh", "lha", "z", "sit", "sitx",
    ];
    for ext in archive_extensions {
        map.insert(ext, Category::Archives);
    }

    // Code - ソースコード・設定ファイル
    let code_extensions = [
        "rs", "py", "js", "ts", "jsx", "tsx", "html", "htm", "css", "scss", "sass", "less",
        "json", "xml", "yaml", "yml", "toml", "md", "markdown", "sh", "bash", "zsh", "fish",
        "c", "cpp", "cc", "cxx", "h", "hpp", "hxx", "go", "java", "kt", "kts", "scala", "rb",
        "php", "pl", "pm", "swift", "m", "mm", "sql", "r", "lua", "vim", "el", "clj", "cljs",
        "edn", "ex", "exs", "erl", "hrl", "hs", "lhs", "ml", "mli", "fs", "fsi", "fsx",
        "dockerfile", "makefile", "cmake", "gradle", "sbt", "cabal",
    ];
    for ext in code_extensions {
        map.insert(ext, Category::Code);
    }

    map
});

/// 拡張子からカテゴリを取得する
///
/// # Arguments
/// * `extension` - ファイルの拡張子（ドットなし、大文字小文字は問わない）
///
/// # Returns
/// 対応するカテゴリ。マッピングに存在しない場合は `Category::Others` を返す。
pub fn get_category(extension: &str) -> Category {
    let ext_lower = extension.to_lowercase();
    EXTENSION_MAP
        .get(ext_lower.as_str())
        .copied()
        .unwrap_or(Category::Others)
}

/// 拡張子なしのファイルに対するデフォルトカテゴリ
pub fn get_default_category() -> Category {
    Category::Others
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_extensions() {
        assert_eq!(get_category("jpg"), Category::Images);
        assert_eq!(get_category("PNG"), Category::Images); // 大文字テスト
        assert_eq!(get_category("HEIC"), Category::Images);
    }

    #[test]
    fn test_video_extensions() {
        assert_eq!(get_category("mp4"), Category::Videos);
        assert_eq!(get_category("MKV"), Category::Videos);
    }

    #[test]
    fn test_document_extensions() {
        assert_eq!(get_category("pdf"), Category::Documents);
        assert_eq!(get_category("docx"), Category::Documents);
    }

    #[test]
    fn test_music_extensions() {
        assert_eq!(get_category("mp3"), Category::Music);
        assert_eq!(get_category("flac"), Category::Music);
    }

    #[test]
    fn test_archive_extensions() {
        assert_eq!(get_category("zip"), Category::Archives);
        assert_eq!(get_category("tar"), Category::Archives);
    }

    #[test]
    fn test_code_extensions() {
        assert_eq!(get_category("rs"), Category::Code);
        assert_eq!(get_category("py"), Category::Code);
        assert_eq!(get_category("js"), Category::Code);
    }

    #[test]
    fn test_unknown_extension() {
        assert_eq!(get_category("xyz"), Category::Others);
        assert_eq!(get_category("unknown"), Category::Others);
    }

    #[test]
    fn test_folder_name() {
        assert_eq!(Category::Images.folder_name(), "Images");
        assert_eq!(Category::Others.folder_name(), "Others");
    }
}

