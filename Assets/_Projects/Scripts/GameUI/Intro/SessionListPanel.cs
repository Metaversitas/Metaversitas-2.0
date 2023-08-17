using System.Collections.Generic;
using Fusion;
using UIComponents;
using UnityEngine;
using UnityEngine.UI;

namespace GameUI.Intro
{
	public class SessionListPanel : MonoBehaviour
	{
		[SerializeField] private Text _header;
		[SerializeField] private NewSessionPanel _newSessionPanel;
		[SerializeField] private GridBuilder _sessionGrid;
		[SerializeField] private SessionListItem _sessionListItemPrefab;
		[SerializeField] private Text _error;

		private PlayMode _playMode;
		private App _app;

		public async void Show(PlayMode mode)
		{
			gameObject.SetActive(true);
			_playMode = mode;
			_error.text = "";
			_header.text = $"{mode} Lobby";
			_app = App.FindInstance();
			OnSessionListUpdated(new List<SessionInfo>());
			await _app.EnterLobby($"GameMode{mode}", OnSessionListUpdated);
		}

		public void Hide()
		{
			_newSessionPanel.Hide();
			_app?.Disconnect();
			gameObject.SetActive(false);
		}

		public void OnSessionListUpdated(List<SessionInfo> sessions)
		{
			_sessionGrid.BeginUpdate();
			if (sessions != null)
			{
				foreach (SessionInfo info in sessions)
				{
					_sessionGrid.AddRow(_sessionListItemPrefab, item => item.Setup(info, selectedSession =>
					{
						// Join an existing session - this will unload the current scene and take us to the Staging area
						_app.JoinSession(selectedSession);
					}));
				}
			}
			else
			{
				Hide();
				_error.text = "Failed to join lobby";
			}
			_sessionGrid.EndUpdate();
		}
		
		public void OnShowNewSessionUI()
		{
			_newSessionPanel.Show(_playMode);
		}
	}
}