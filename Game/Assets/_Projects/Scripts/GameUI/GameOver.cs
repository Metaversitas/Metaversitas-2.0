using UnityEngine;

namespace GameUI
{
	public class GameOver : MonoBehaviour
	{
		public void OnContinue()
		{
			App.FindInstance().Session.LoadMap(MapIndex.Lobby);
		}
	}
}