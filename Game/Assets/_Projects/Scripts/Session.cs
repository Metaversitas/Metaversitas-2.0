using Fusion;
using UnityEngine;

/// <summary>
/// Session gets created when a game session starts and exists in only one instance.
/// It survives scene loading and can be used to control game-logic inside a session, across scenes.
/// </summary>

public class Session : NetworkBehaviour
{
	[Networked] public TickTimer PostLoadCountDown { get; set; }
	
	public SessionProps Props => new SessionProps(Runner.SessionInfo.Properties);
	public SessionInfo Info => Runner.SessionInfo;
	
	public Map Map { get; set; }

	private App _app;

	public override void Spawned()
	{
		_app = App.FindInstance();
		_app.Session = this;
		
		if (Object.HasStateAuthority && (Runner.CurrentScene == 0 || Runner.CurrentScene == SceneRef.None))
		{
			PostLoadCountDown = TickTimer.None;

			if (_app.SkipStaging) {
				LoadMap(_app.AutoSession.StartMap);
			} else {
				if (_app.ChangeSceneOnConnected)
				{
					Runner.SetActiveScene(_app.FirstConnectedScene);
				}
			}
		}
	}

	[Rpc(RpcSources.All, RpcTargets.StateAuthority, Channel = RpcChannel.Reliable)]
	public RpcInvokeInfo RPC_FinishedLoading(PlayerRef playerRef)
	{
		Debug.Log($"RPC finished loading!");

		int done = 0, total = 0;
		_app.ForEachPlayer(ply =>
		{
			if (ply.Object.InputAuthority == playerRef)
				ply.DoneLoading = true;
			if (ply.DoneLoading)
				done++;
			total++;
		});

		Debug.Log($"{done} of {total} finished loading!");
		
		if ( done >= total && !PostLoadCountDown.Expired(Runner))
		{
			PostLoadCountDown = TickTimer.CreateFromSeconds(Runner,3);
		}
		return default;
	}

	public void LoadMap(MapIndex mapIndex)
	{
		_app.ForEachPlayer(ply =>
		{
			ply.DoneLoading = false;
		});
		Runner.SessionInfo.IsOpen = Props.AllowLateJoin;
		Runner.SetActiveScene((int)mapIndex);
	}
}