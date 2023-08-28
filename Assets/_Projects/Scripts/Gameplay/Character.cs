using Fusion;
using UnityEngine;
using UnityEngine.UI;

/// <summary>
/// Visual representation of a Player - the Character is instantiated by the map once it's loaded.
/// This class handles camera tracking and player movement and is destroyed when the map is unloaded.
/// (I.e. the player gets a new avatar in each map)
/// </summary>

public class Character : NetworkBehaviour
{
	[SerializeField] private Text _name;
	[SerializeField] private Animator _animator;
	[SerializeField] private MeshRenderer _mesh;
	[SerializeField] private CharacterCamera characterCamera;
	[SerializeField] private CharacterInteraction _interaction;

	public float moveVelocity = 5f;
	public float maxYDegrees = 70f;
	public float mouseSensitivity = 2f;
	private float verticalRotation = 0;

	[UnityHeader("Networked Anim Field")]
	[Networked] public int yRotation { get; set; }
	[Networked] public int xMovement { get; set; }
	[Networked] public int yMovement { get; set; }

	[Networked] public Player Player { get; set; }

	private Transform _camTransform;
	private CursorLock _cursorLock;
	[Networked]
	private bool _isReadInput { get; set; }

    private void Awake()
    {
		_cursorLock = GetComponent<CursorLock>();
		_cursorLock.ToggleCursorLock();
    }

    public override void Spawned()
	{
		_isReadInput = false;

		if (HasInputAuthority)
		{
			App.FindInstance().ShowPlayerSetup();
			characterCamera.SetCameraParent(transform);
			_camTransform = Camera.main.transform;
		}
	}

	public void SetPlayer(Player player)
    {
		Player = player;
		_interaction.Player = player;
    }

    private void Update()
	{
		if (Object.HasInputAuthority == false) return;
		if (_cursorLock.IsLocked == false) return;
		ControlCameraUsingMouse();
    }

    private void ControlCameraUsingMouse()
    {
        float mouseX = Input.GetAxis("Mouse X") * mouseSensitivity;
        float mouseY = Input.GetAxis("Mouse Y") * mouseSensitivity;

        verticalRotation -= mouseY;
        verticalRotation = Mathf.Clamp(verticalRotation, -maxYDegrees, maxYDegrees);

		// rotate transform locally
        transform.Rotate(Vector3.up * mouseX);
        _camTransform.localRotation = Quaternion.Euler(verticalRotation, 0, 0);
    }

    public void LateUpdate()
	{
		// This is a little brute-force, but it gets the job done.
		// Could use an OnChanged listener on the properties instead.
		_name.text = Player.Name.Value;
		_mesh.material.color = Player.Color;
	}

	public override void FixedUpdateNetwork()
	{
		if (Player == null) return;

		if (Player.InputEnabled && GetInput(out InputData data))
		{
			_isReadInput = true;
			if (data.GetButton(ButtonFlag.LEFT))
            {
				transform.position -= Runner.DeltaTime * moveVelocity * transform.right;
				xMovement = -1;
			} 
			else if (data.GetButton(ButtonFlag.RIGHT))
            {
				transform.position += Runner.DeltaTime * moveVelocity * transform.right;
				xMovement = 1;
			} 
			else if (data.GetButton(ButtonFlag.FORWARD))
            {
				transform.position += Runner.DeltaTime * moveVelocity * transform.forward;
				yMovement = 1;
			} 
			else if (data.GetButton(ButtonFlag.BACKWARD))
            {
				transform.position -= Runner.DeltaTime * moveVelocity * transform.forward;
				yMovement = -1;
			}
			else // No input
			{
				_isReadInput = false;
				xMovement = 0;
				yMovement = 0;
			}
		}

		// set y rot networked if ourself
		if (Object.HasInputAuthority) yRotation = Mathf.CeilToInt(transform.rotation.eulerAngles.y);
	}

    public override void Render()
    {
		if(_isReadInput)
        {
			_animator.SetFloat("xMovement", xMovement);
			_animator.SetFloat("yMovement", yMovement);
        }
        else
		{
			_animator.SetFloat("xMovement", 0);
            _animator.SetFloat("yMovement", 0);
        }

		// render rotation if not ourself
		if (Object.HasInputAuthority) return;
		transform.Rotate(Vector3.up * yRotation);
    }
}