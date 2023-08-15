public interface ISceneService
{
    string ServiceId { get; }
    SceneContext Context { get; }
    bool IsActive { get; }
    bool IsInitialized { get; }

    void Initialize(SceneContext context);
    void Deinitialize();
    void Activate();
    void Deactivate();
    void Tick();
}