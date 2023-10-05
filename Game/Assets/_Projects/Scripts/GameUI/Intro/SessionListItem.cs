using System;
using Fusion;
using UIComponents;
using UnityEngine;
using UnityEngine.UI;

namespace GameUI.Intro
{
	public class SessionListItem : GridCell
	{
		[SerializeField] private Text _name;
		[SerializeField] private Text _map;
		[SerializeField] private Text _players;
        [SerializeField] private InputField _pass;

        private Action<SessionInfo> _onJoin;
		private SessionInfo _info;
        private SessionProps _props;

		public void Setup(SessionInfo info, Action<SessionInfo> onJoin)
		{
			_info = info;
			_name.text = $"{info.Name} ({info.Region})";
			_map.text = $"Map {new SessionProps(info.Properties).StartMap}";
			_players.text = $"{info.PlayerCount}/{info.MaxPlayers}";
			_onJoin = onJoin;

            if (_props == null)
            {
                _props = new SessionProps(info.Properties);
            }
        }

        public void OnJoin()
        {
            if (_props.RoomPass == _pass.text)
            {
                _onJoin(_info);
            }
            else
            {
                Debug.Log("Password Invalid");
            }

        }
    }
}