using UnityEngine;
using UnityEngine.UI;

[RequireComponent(typeof(CanvasGroup))]
[RequireComponent(typeof(Canvas))]
[RequireComponent(typeof(GraphicRaycaster))]
public abstract class UIPageView : UIWidget
{
    // PUBLIC VARIABLE / PROPERTY
    public bool IsOpen { get; protected set; }
    public bool IsInteractable { get { return CanvasGroup.interactable; } set { CanvasGroup.interactable = value; } }
    public int Priority => _priority;

    // PRIVATE INSPECTOR VARIABLE
    [SerializeField]
    private int _priority;

    // PRIVATE HIDDEN VARIABLE
    private CanvasGroup canvasGroup;
    private Canvas canvas;

    public CanvasGroup CanvasGroup => canvasGroup == null ? canvasGroup = GetComponent<CanvasGroup>() : canvasGroup;
    public Canvas Canvas => canvas == null ? canvas = GetComponent<Canvas>() : canvas;

    // MONOBEHAVIOUR METHOD

    // OVERRIDE METHOD
    protected override void OnInitialize()
    {

    }

    protected override void OnDeinitialize()
    {

    }

    // PUBLIC METHOD
    public void Open()
    {
        SceneUI.Open(this);
    }

    public void Close()
    {
        if (SceneUI == null)
        {
            Debug.Log($"Closing view {gameObject.name} without SceneUI");
            CloseInternal();
        }
        else
        {
            SceneUI.Close(this);
        }
    }

    public void ForceClose()
    {
        OnClosed();
    }

    internal void OpenInternal()
    {
        if (IsOpen)
            return;

        IsOpen = true;
        Visible();
        OnOpen();
    }

    internal void CloseInternal()
    {
        if (!IsOpen)
            return;

        IsOpen = false;
        Hidden();
        OnClosed();
    }

    // VIRTUAL METHOD
    protected virtual void OnOpen() { }
    protected virtual void OnClosed() { }

    // PROTECTED METHODS
    protected T Switch<T>() where T : UIPageView
    {
        Close();

        return SceneUI.Open<T>();
    }

    protected T Open<T>() where T : UIPageView
    {
        return SceneUI.Open<T>();
    }

    protected void Open(UIPageView view)
    {
        SceneUI.Open(view);
    }
}
