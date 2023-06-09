import FieldKind from "../../../field/kind";
import { BENCHER_API_URL } from "../../../site/util";
import TESTBED_FIELDS from "./fields/testbed";
import { Button, Card, Display, Operation, Row } from "../types";
import { parentPath, addPath, viewSlugPath } from "../util";

const testbedsConfig = {
	[Operation.LIST]: {
		operation: Operation.LIST,
		header: {
			title: "Testbeds",
			buttons: [
				{
					kind: Button.ADD,
					path: addPath,
				},
				{ kind: Button.REFRESH },
			],
		},
		table: {
			url: (path_params) =>
				`${BENCHER_API_URL()}/v0/projects/${
					path_params?.project_slug
				}/testbeds`,
			add: {
				prefix: (
					<div>
						<h4>
							🐰 Goodbye, <code>localhost</code>!
						</h4>
						<p>
							It's easy to add a new testbed.
							<br />
							Tap below to get started.
						</p>
					</div>
				),
				path: addPath,
				text: "Add a Testbed",
			},
			row: {
				key: "name",
				items: [
					{
						kind: Row.TEXT,
						key: "slug",
					},
					{},
					{},
					{},
				],
				button: {
					text: "View",
					path: (pathname, datum) => viewSlugPath(pathname, datum),
				},
			},
		},
	},
	[Operation.ADD]: {
		operation: Operation.ADD,
		header: {
			title: "Add Testbed",
			path: parentPath,
		},
		form: {
			url: (path_params) =>
				`${BENCHER_API_URL()}/v0/projects/${
					path_params?.project_slug
				}/testbeds`,
			fields: [
				{
					kind: FieldKind.INPUT,
					label: "Name",
					key: "name",
					value: "",
					valid: null,
					validate: true,
					config: TESTBED_FIELDS.name,
				},
			],
			path: parentPath,
		},
	},
	[Operation.VIEW]: {
		operation: Operation.VIEW,
		header: {
			key: "name",
			path: parentPath,
		},
		deck: {
			url: (path_params) =>
				`${BENCHER_API_URL()}/v0/projects/${
					path_params?.project_slug
				}/testbeds/${path_params?.testbed_slug}`,
			cards: [
				{
					kind: Card.FIELD,
					label: "Testbed Name",
					key: "name",
					display: Display.RAW,
				},
				{
					kind: Card.FIELD,
					label: "Testbed Slug",
					key: "slug",
					display: Display.RAW,
				},
				{
					kind: Card.FIELD,
					label: "Testbed UUID",
					key: "uuid",
					display: Display.RAW,
				},
			],
		},
	},
};

export default testbedsConfig;
