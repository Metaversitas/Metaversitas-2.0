using UnityEditor;
using UnityEditor.SceneManagement;
using UnityEngine;

public class SceneShortcut : MonoBehaviour
{
    [MenuItem("Tools/Utils/OpenScene/MainMenu #1")]
    public static void OpenMainMenu()
    {
        OpenScene("MainMenu");
    }

    [MenuItem("Tools/Utils/OpenScene/Lobby #2")]
    public static void OpenLobby()
    {
        OpenScene("Lobby");
    }

    [MenuItem("Tools/Utils/OpenScene/Lab Fisika #3")]
    public static void OpenLabFisika()
    {
        OpenScene("Lab Fisika");
    }

    [MenuItem("Tools/Utils/OpenScene/Borobudur #4")]
    public static void OpenBorobudur()
    {
        OpenScene("Borobudur");
    }

    [MenuItem("Tools/Utils/OpenScene/Museum #5")]
    public static void OpenMetaLand()
    {
        OpenScene("MetaLand");
    }

    private static void OpenScene(string sceneName)
    {
        EditorSceneManager.SaveCurrentModifiedScenesIfUserWantsTo();
        EditorSceneManager.OpenScene(string.Format(@"Assets\_Projects\Scenes\{0}.unity", sceneName));
    }
}
