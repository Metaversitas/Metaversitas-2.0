
using UnityEngine;

public interface IContextBehaviour
{
    SceneContext Context { get; set; }
}

public abstract class ContextBehaviour : MonoBehaviour, IContextBehaviour
{
    public SceneContext Context { get; set; }
}