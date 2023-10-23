using System.Text;
using UIComponents;
using UnityEngine;
using UnityEngine.UI;

namespace GameUI.Staging
{
	public class Staging : MonoBehaviour
	{
		[SerializeField] private GridBuilder _playerGrid;
		[SerializeField] private PlayerListItem _playerListItemPrefab;

		[SerializeField] private Button _startButton;
		[SerializeField] private Text _startLabel;
		[SerializeField] private Text _sessionInfo;

		private float _sessionRefresh;
		private App _app;

		private void Awake()
		{
			_app = App.FindInstance();
			_app.GetPlayer()?.RPC_SetIsReady(false);
		}

		void Update()
		{
			_playerGrid.BeginUpdate();
	  
			_playerGrid.EndUpdate();

			if (_sessionRefresh <= 0)
			{
				UpdateSessionInfo();
				_sessionRefresh = 2.0f;
			}
			_sessionRefresh -= Time.deltaTime;
		}

		public void OnStart()
		{
			SessionProps props = _app.Session.Props;
			_app.Session.LoadMap(props.StartMap);
		}

		public void OnDisconnect()
		{
			_app.Disconnect();
		}

		private void UpdateSessionInfo()
		{
			Session s = _app.Session;
			StringBuilder sb = new StringBuilder();
			if (s != null)
			{
				sb.AppendLine($"Session Name: {s.Info.Name}");
				sb.AppendLine($"Region: {s.Info.Region}");
				sb.AppendLine($"Game Type: {s.Props.PlayMode}");
				sb.AppendLine($"Map: {s.Props.StartMap}");
				sb.AppendLine($"Password: {s.Props.RoomPass}");
				sb.AppendLine($"Pertemuan: {s.Props.RoomPertemuan}");
			}
			_sessionInfo.text = sb.ToString();
		}
	}
}
