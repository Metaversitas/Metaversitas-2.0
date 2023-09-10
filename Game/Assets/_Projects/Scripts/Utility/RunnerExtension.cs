using Fusion;

namespace Utility
{
	public static class RunnerExtension
	{
		public static Player GetPlayer(this NetworkRunner runner)
		{
			return runner?.GetPlayerObject(runner.LocalPlayer)?.GetComponent<Player>();			
		}
	}
}