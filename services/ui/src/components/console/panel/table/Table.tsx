import { Link, useLocation, useNavigate } from "solid-app-router";
import { For, Switch, Match, createMemo } from "solid-js";
import { Row } from "../../config/types";

const Table = (props) => {
	const location = useLocation();
	const pathname = createMemo(() => location.pathname);

	return (
		<>
			{props.table_data()?.length === 0 ? (
				<div class="box">
					<AddButton pathname={pathname} add={props.config?.add} />
				</div>
			) : (
				<div class="pricing-table is-horizontal">
					<For each={props.table_data()}>
						{(datum, i) => (
							<div class="pricing-plan is-primary">
								<div class="plan-header">
									<p style="overflow-wrap:anywhere;">
										{datum[props.config?.row?.key]}
									</p>
								</div>
								<div class="plan-items">
									<For each={props.config?.row?.items}>
										{(item, i) => (
											<div class="plan-item">
												<p style="overflow-wrap:anywhere;">
													<Switch fallback="-">
														<Match when={item.kind === Row.TEXT}>
															{datum[item.key]}
														</Match>
														<Match when={item.kind === Row.BOOL}>
															{item.text}: {datum[item.key] ? "true" : "false"}
														</Match>
														<Match when={item.kind === Row.SELECT}>
															{item.value?.options.reduce((field, option) => {
																if (datum[item.key] === option.value) {
																	return option.option;
																} else {
																	return field;
																}
															}, datum[item.key])}
														</Match>
													</Switch>
												</p>
											</div>
										)}
									</For>
								</div>
								<div class="plan-footer">
									<RowButton
										config={props.config?.row?.button}
										pathname={pathname}
										datum={datum}
									/>
								</div>
							</div>
						)}
					</For>
				</div>
			)}
		</>
	);
};

const AddButton = (props) => {
	return (
		<>
			<div class="content has-text-centered">{props.add?.prefix}</div>
			<Link
				class="button is-primary is-fullwidth"
				href={props.add?.path(props.pathname())}
			>
				{props.add?.text}
			</Link>
		</>
	);
};

const RowButton = (props) => {
	const navigate = useNavigate();

	return (
		<button
			class="button is-fullwidth"
			onClick={(e) => {
				e.preventDefault();
				navigate(props.config?.path?.(props.pathname(), props.datum));
			}}
		>
			{props.config?.text}
		</button>
	);
};

export default Table;
