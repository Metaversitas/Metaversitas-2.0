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
	[SerializeField] private CharacterInteraction _interaction;

	public float moveVelocity = 5f;

	[UnityHeader("Networked Anim Field")]
	[Networked] public Angle yCamRotation { get; set; }
	[Networked] public int xMovement { get; set; }
	[Networked] public int yMovement { get; set; }

	[Networked] public Player Player { get; set; }

	[Networked]
	private bool _isReadInput { get; set; }


    public override void Spawned()
	{
		_isReadInput = false;

		if (HasInputAuthority)
		{
			App.FindInstance().ShowPlayerSetup();
		}
	}

	public void SetPlayer(Player player)
    {
		Player = player;
		_interaction.Player = player;
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

			yCamRotation += data.YCamRotation;
		}

        transform.rotation = Quaternion.Euler(0, (float)yCamRotation, 0);
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
    }
}