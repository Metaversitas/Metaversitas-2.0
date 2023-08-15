using UnityEngine;

/// <summary>
/// This class is to wrap your in-game runtime references
/// Assign it manually via inspector, or change the way you want.
/// The references itself can be anything
/// Might be a SceneService, Database handler, any object you want to refer.
/// </summary>
[System.Serializable]
public class SceneContext
{
    public SceneUI UI;
}
