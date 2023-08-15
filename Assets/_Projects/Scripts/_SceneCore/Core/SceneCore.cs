using System.Collections;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

public class SceneCore : MonoBehaviour
{
	public bool IsContextReady { get; private set; }
	public bool IsActive { get; private set; }
	public SceneContext Context => _context;

	[SerializeField]
	private bool _selfInitialize;
	[SerializeField]
	private SceneContext _context;

	protected bool _isInitialized;

	private Dictionary<string, ISceneService> _services = new Dictionary<string, ISceneService>();

	// PUBLIC METHODS

	public void PrepareContext()
	{
		if (IsContextReady) return;
		OnPrepareContext(_context);
		IsContextReady = true;
	}

	public void Initialize(SceneContext context = null)
	{

#if UNITY_SERVER
		Application.targetFrameRate = 30;
#else
		Application.targetFrameRate = 60;
#endif

		if (_isInitialized) return;
		if (context != null) _context = context;

		PrepareContext();
		CollectServices();
		OnInitialize();
		_isInitialized = true;
	}

	public void Deinitialize()
	{
		if (!_isInitialized) return;
		Deactivate();
		OnDeinitialize();
		_isInitialized = false;
	}

	public IEnumerator Activate()
	{
		if (!_isInitialized) yield break;
		yield return OnActivate();
		IsActive = true;
	}

	public void Deactivate()
	{
		if (!IsActive) return;
		OnDeactivate();
		IsActive = false;
	}

	public T GetService<T>(string name) where T : ISceneService
	{
		var isValueExist = _services.TryGetValue(name, out var sceneService);
		return isValueExist ? (T)sceneService : default;
	}

	public void Quit()
	{
		Deinitialize();

#if UNITY_EDITOR
		UnityEditor.EditorApplication.isPlaying = false;
#else
		Application.Quit();
#endif
	}

	// MONOBEHAVIOUR

	protected virtual void Awake()
	{
		if (_selfInitialize)
		{
			Initialize();
		}
	}

	protected IEnumerator Start()
	{
		if (!_isInitialized) yield break;

		if (_selfInitialize && !IsActive)
		{
			// UI cannot be initialized in Awake, Canvas elements need to Awake first
			AddService(_context.UI);

			yield return Activate();
		}
	}

	protected virtual void Update()
	{
		if (!IsActive) return;
		OnTick();
	}

	protected void OnDestroy()
	{
		Deinitialize();
	}

	protected void OnApplicationQuit()
	{
		Deinitialize();
	}

	// PROTECTED METHODS

	protected virtual void OnPrepareContext(SceneContext context)
	{

	}

	protected virtual void CollectServices()
	{
		var services = GetComponentsInChildren<ISceneService>(true);
		foreach (var service in services)
		{
			AddService(service);
		}
	}

	protected virtual void OnInitialize()
	{
		for (int i = 0; i < _services.Count; i++)
		{
			_services.ElementAt(i).Value.Initialize(Context);
		}
	}

	protected virtual IEnumerator OnActivate()
	{
		for (int i = 0; i < _services.Count; i++)
		{
			_services.ElementAt(i).Value.Activate();
		}

		yield break;
	}

	protected virtual void OnTick()
	{
		for (int i = 0, count = _services.Count; i < count; i++)
		{
			_services.ElementAt(i).Value.Tick();
		}
	}

	protected virtual void OnDeactivate()
	{
		for (int i = 0; i < _services.Count; i++)
		{
			_services.ElementAt(i).Value.Deactivate();
		}
	}

	protected virtual void OnDeinitialize()
	{
		for (int i = 0; i < _services.Count; i++)
		{
			_services.ElementAt(i).Value.Deinitialize();
		}

		_services.Clear();
	}

	public void AddService(ISceneService service)
	{
		if (service == null)
		{
			Debug.LogError($"Missing service");
			return;
		}

		if (_services.ContainsKey(service.ServiceId) == true)
		{
			Debug.LogError($"Service {service} already added.");
			return;
		}

		_services.Add(service.ServiceId, service);

		if (_isInitialized == true)
		{
			service.Initialize( Context);
		}

		if (IsActive == true)
		{
			service.Activate();
		}
	}

	public void RemoveService(SceneService service)
	{
		if (service == null)
		{
			Debug.LogError($"Missing service");
			return;
		}

		if (_services.ContainsValue(service) == true)
		{
			_services.Remove(nameof(service));
			return;
		}
	}
}
