using Fusion;
using UnityEngine;

/// <summary>
/// Player is a network object that represents a players core data. One instance is spawned
/// for each player when the game session starts and it lives until the session ends.
/// This is not the visual representation of the player.
/// </summary>

[OrderBefore(typeof(Character))]
public class Player : NetworkBehaviour
{
	[SerializeField] public Character CharacterPrefab;
	
	[Networked] public NetworkString<_32> Name { get; set; }
	[Networked] public Color Color { get; set; }
	[Networked] public NetworkBool Ready { get; set; }
	[Networked] public NetworkBool DoneLoading { get; set; }

	public bool InputEnabled => _app.AllowInput;

	private Character _character;
	private App _app;

	public override void Spawned()
	{
		_app = App.FindInstance();
		// Make sure we go down with the runner and that we're not destroyed onload unless the runner is!
		transform.SetParent(Runner.gameObject.transform);
	}

	public override void FixedUpdateNetwork()
	{
		if (HasStateAuthority && _character == null && _app!=null && _app.Session!=null && _app.Session.Map)
		{
			Debug.Log($"Spawning avatar for player {Name} with input auth {Object.InputAuthority}");
			Transform t = _app.Session.Map.GetSpawnPoint(Object.InputAuthority);
			_character = Runner.Spawn(CharacterPrefab, t.position, t.rotation, Object.InputAuthority, (runner, o) =>
			{
				Character character = o.GetComponent<Character>();
				Debug.Log($"Created Character for Player {Name}");
				character.Player = this;
			});
		}
	}
	
	public void Despawn()
	{
		if (HasStateAuthority)
		{
			if (_character != null)
			{
				Runner.Despawn(_character.Object);
				_character = null;
			}
			Runner.Despawn(Object);
		}
	}

	[Rpc(RpcSources.InputAuthority, RpcTargets.StateAuthority)]
	public void RPC_SetIsReady(NetworkBool ready)
	{
		Ready = ready;
	}

	[Rpc(RpcSources.InputAuthority, RpcTargets.StateAuthority)]
	public void RPC_SetName(NetworkString<_32> name)
	{
		Name = name;
	}

	[Rpc(RpcSources.InputAuthority, RpcTargets.StateAuthority)]
	public void RPC_SetColor(Color color)
	{
		Color = color;
	}
}