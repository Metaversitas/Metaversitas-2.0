using UnityEngine;

public abstract class SceneService : MonoBehaviour, ISceneService
{
    public string ServiceId => GetId();
    public SceneContext Context => _context;
    public bool IsActive => _isActive;
    public bool IsInitialized => _isInitialized;

    private SceneContext _context;
    private bool _isInitialized;
    private bool _isActive;

    public void Initialize(SceneContext context)
    {
        if (_isInitialized) return;

        _context = context;

        OnInitialize();
        _isInitialized = true;
    }

    public void Deinitialize()
    {
        if (!_isInitialized) return;

        Deactivate();
        OnDeinitialize();

        _context = null;
        _isInitialized = false;
    }

    public void Activate()
    {
        if (!_isInitialized) return;
        if (_isActive) return;

        OnActivate();
        _isActive = true;
    }

    public void Tick()
    {
        if (!_isActive) return;
        OnTick();
    }

    public void Deactivate()
    {
        if (!_isActive) return;

        OnDeactivate();
        _isActive = false;
    }

    protected abstract string GetId();

    protected virtual void OnInitialize() { }

    protected virtual void OnDeinitialize() { }

    protected virtual void OnActivate() { }

    protected virtual void OnDeactivate() { }

    protected virtual void OnTick() { }
}
